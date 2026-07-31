//! OpenAI-compatible chat completions client (OpenAI, Groq, OpenRouter, Ollama, vLLM, …).

use crate::message::{ChatMessage, ToolDefinition};
use crate::provider::{LlmProvider, ProviderId};
use crate::request::{CompletionRequest, ResponseFormat};
use crate::response::{CompletionResponse, FinishReason, TokenUsage, ToolCallRequest};
use async_trait::async_trait;
use clawrs_core::{ClawrsError, ClawrsResult, ToolCallId};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct OpenAiCompatibleConfig {
    pub base_url: String,
    pub api_key: Option<String>,
    pub provider_id: ProviderId,
    pub default_model: String,
}

impl OpenAiCompatibleConfig {
    pub fn ollama(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            api_key: None,
            provider_id: ProviderId::new("ollama"),
            default_model: model.into(),
        }
    }
}

pub struct OpenAiCompatibleProvider {
    config: OpenAiCompatibleConfig,
    http: Client,
}

impl OpenAiCompatibleProvider {
    pub fn new(config: OpenAiCompatibleConfig) -> ClawrsResult<Self> {
        let http = Client::builder()
            .user_agent(format!("clawrs/{}", clawrs_core::VERSION))
            .build()
            .map_err(|e| ClawrsError::internal(format!("http client: {e}")))?;
        Ok(Self { config, http })
    }
}

#[derive(Serialize)]
struct ApiRequest<'a> {
    model: &'a str,
    messages: &'a [ChatMessage],
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    stream: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<ApiTool<'a>>,
}

#[derive(Serialize)]
struct ApiTool<'a> {
    r#type: &'static str,
    function: ApiFunction<'a>,
}

#[derive(Serialize)]
struct ApiFunction<'a> {
    name: &'a str,
    description: &'a str,
    parameters: &'a serde_json::Value,
}

#[derive(Deserialize)]
struct ApiResponse {
    model: String,
    choices: Vec<ApiChoice>,
    usage: Option<ApiUsage>,
}

#[derive(Deserialize)]
struct ApiChoice {
    message: ApiMessage,
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct ApiMessage {
    content: Option<String>,
    tool_calls: Option<Vec<ApiToolCall>>,
}

#[derive(Deserialize)]
struct ApiToolCall {
    id: String,
    function: ApiToolCallFn,
}

#[derive(Deserialize)]
struct ApiToolCallFn {
    name: String,
    arguments: String,
}

#[derive(Deserialize)]
struct ApiUsage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

fn map_tools(tools: &[ToolDefinition]) -> Vec<ApiTool<'_>> {
    tools
        .iter()
        .map(|t| ApiTool {
            r#type: "function",
            function: ApiFunction {
                name: &t.name,
                description: &t.description,
                parameters: &t.parameters_schema,
            },
        })
        .collect()
}

fn map_finish_reason(reason: Option<&str>) -> FinishReason {
    match reason {
        Some("length") => FinishReason::Length,
        Some("tool_calls") => FinishReason::ToolCalls,
        Some("content_filter") => FinishReason::ContentFilter,
        Some("stop") | None => FinishReason::Stop,
        Some(_) => FinishReason::Stop,
    }
}

#[async_trait]
impl LlmProvider for OpenAiCompatibleProvider {
    fn id(&self) -> ProviderId {
        self.config.provider_id.clone()
    }

    async fn complete(&self, request: CompletionRequest) -> ClawrsResult<CompletionResponse> {
        let model = if request.model.is_empty() {
            self.config.default_model.as_str()
        } else {
            request.model.as_str()
        };

        let api_req = ApiRequest {
            model,
            messages: &request.messages,
            temperature: request.options.temperature,
            max_tokens: request.options.max_tokens,
            stream: false,
            tools: map_tools(&request.tools),
        };

        let url = format!(
            "{}/v1/chat/completions",
            self.config.base_url.trim_end_matches('/')
        );

        let mut builder = self.http.post(url).json(&api_req);
        if let Some(key) = &self.config.api_key {
            builder = builder.bearer_auth(key);
        }

        if matches!(request.response_format, ResponseFormat::JsonObject) {
            // OpenAI uses response_format; kept minimal until structured output expands.
        }

        let response = builder.send().await.map_err(|e| ClawrsError::Provider {
            provider: self.config.provider_id.0.clone(),
            message: format!("request failed: {e}"),
        })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(ClawrsError::Provider {
                provider: self.config.provider_id.0.clone(),
                message: format!("HTTP {status}: {body}"),
            });
        }

        let parsed: ApiResponse = response.json().await.map_err(|e| ClawrsError::Provider {
            provider: self.config.provider_id.0.clone(),
            message: format!("invalid JSON: {e}"),
        })?;

        let choice = parsed.choices.into_iter().next().ok_or_else(|| {
            ClawrsError::Provider {
                provider: self.config.provider_id.0.clone(),
                message: "empty choices".into(),
            }
        })?;

        let tool_calls = choice
            .message
            .tool_calls
            .unwrap_or_default()
            .into_iter()
            .filter_map(|tc| {
                let args: serde_json::Value =
                    serde_json::from_str(&tc.function.arguments).unwrap_or(serde_json::Value::Null);
                Some(ToolCallRequest {
                    id: ToolCallId::from_external(&tc.id),
                    name: tc.function.name,
                    arguments: args,
                })
            })
            .collect();

        let usage = parsed.usage.unwrap_or(ApiUsage {
            prompt_tokens: 0,
            completion_tokens: 0,
            total_tokens: 0,
        });

        Ok(CompletionResponse {
            content: choice.message.content.unwrap_or_default(),
            tool_calls,
            finish_reason: map_finish_reason(choice.finish_reason.as_deref()),
            usage: TokenUsage {
                prompt_tokens: usage.prompt_tokens,
                completion_tokens: usage.completion_tokens,
                total_tokens: usage.total_tokens,
            },
            model: parsed.model,
        })
    }
}
