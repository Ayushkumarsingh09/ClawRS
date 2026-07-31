use crate::kind::AgentKind;
use crate::prompt::PromptMode;
use clawrs_core::{AgentId, ClawrsResult, SessionId, TenantContext};
use clawrs_events::{EventBus, EventEnvelope, EventKind};
use clawrs_llm::{ChatMessage, CompletionRequest, LlmProvider, MessageRole, RequestOptions, ResponseFormat};
use clawrs_memory::{MemoryQuery, MemoryRecord, MemoryStore, MemoryTier};
use clawrs_tools::{ToolContext, ToolRegistry};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, instrument};

use crate::pipeline::{AgentPipeline, PipelineState};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentRunInput {
    pub tenant: TenantContext,
    pub agent_id: AgentId,
    pub session_id: SessionId,
    pub kind: AgentKind,
    pub prompt_mode: PromptMode,
    pub model: String,
    pub user_message: String,
    pub max_tool_rounds: u32,
    #[serde(default)]
    pub system_prompt: Option<String>,
    #[serde(default)]
    pub prior_messages: Vec<ChatMessage>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentRunOutput {
    pub assistant_message: String,
    pub messages: Vec<ChatMessage>,
    pub tool_rounds: u32,
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
}

pub struct AgentRunner<B, M, P> {
    pub events: Arc<B>,
    pub memory: Arc<M>,
    pub provider: Arc<P>,
    pub tools: Arc<ToolRegistry>,
    pub pipeline: AgentPipeline,
}

impl<B, M, P> AgentRunner<B, M, P>
where
    B: EventBus,
    M: MemoryStore,
    P: LlmProvider,
{
    pub fn new(events: Arc<B>, memory: Arc<M>, provider: Arc<P>, tools: Arc<ToolRegistry>) -> Self {
        Self {
            events,
            memory,
            provider,
            tools,
            pipeline: AgentPipeline::default(),
        }
    }

    #[instrument(skip(self), fields(agent = %input.agent_id, session = %input.session_id))]
    pub async fn run(&self, input: AgentRunInput) -> ClawrsResult<AgentRunOutput> {
        let run_id = clawrs_core::RunId::new_v7();
        self.events
            .publish(
                EventEnvelope::new(
                    EventKind::AgentRunStarted,
                    input.tenant.tenant_id,
                    serde_json::json!({ "user_message": input.user_message }),
                )
                .with_session(input.session_id)
                .with_agent(input.agent_id)
                .with_run(run_id),
            )
            .await?;

        let mut state = PipelineState::new(input.clone());
        self.pipeline.execute(&mut state).await?;

        let tool_ctx = ToolContext {
            tenant: input.tenant.clone(),
            session_id: input.session_id,
            agent_id: input.agent_id,
            dry_run: false,
        };

        let mut tool_rounds = 0u32;
        let mut total_prompt = 0u32;
        let mut total_completion = 0u32;

        loop {
            let mut request = CompletionRequest {
                model: input.model.clone(),
                messages: state.messages.clone(),
                tools: self
                    .tools
                    .list_metadata()
                    .into_iter()
                    .map(|m| m.to_llm_definition())
                    .collect(),
                options: RequestOptions::default(),
                response_format: ResponseFormat::Text,
            };

            if !input.prompt_mode.includes_tools() {
                request.tools.clear();
            }

            self.events
                .publish(
                    EventEnvelope::new(
                        EventKind::LlmRequestStarted,
                        input.tenant.tenant_id,
                        serde_json::json!({ "model": request.model }),
                    )
                    .with_session(input.session_id)
                    .with_agent(input.agent_id)
                    .with_run(run_id),
                )
                .await?;

            let response = self.provider.complete(request).await?;
            total_prompt += response.usage.prompt_tokens;
            total_completion += response.usage.completion_tokens;

            self.events
                .publish(
                    EventEnvelope::new(
                        EventKind::LlmRequestCompleted,
                        input.tenant.tenant_id,
                        serde_json::json!({ "usage": response.usage }),
                    )
                    .with_session(input.session_id)
                    .with_agent(input.agent_id)
                    .with_run(run_id),
                )
                .await?;

            state.messages.push(ChatMessage::assistant(&response.content));

            if response.tool_calls.is_empty() {
                state.assistant_message = response.content;
                break;
            }

            tool_rounds += 1;
            if tool_rounds > input.max_tool_rounds {
                return Err(clawrs_core::ClawrsError::validation(format!(
                    "exceeded max tool rounds ({})",
                    input.max_tool_rounds
                )));
            }

            for call in response.tool_calls {
                self.events
                    .publish(
                        EventEnvelope::new(
                            EventKind::ToolInvoked,
                            input.tenant.tenant_id,
                            serde_json::json!({ "tool": call.name }),
                        )
                        .with_session(input.session_id)
                        .with_agent(input.agent_id)
                        .with_run(run_id),
                    )
                    .await?;

                let result = self
                    .tools
                    .invoke(&call.name, &tool_ctx, call.arguments.clone())
                    .await?;

                self.events
                    .publish(
                        EventEnvelope::new(
                            EventKind::ToolCompleted,
                            input.tenant.tenant_id,
                            serde_json::json!({ "tool": call.name }),
                        )
                        .with_session(input.session_id)
                        .with_agent(input.agent_id)
                        .with_run(run_id),
                    )
                    .await?;

                state.messages.push(ChatMessage {
                    role: MessageRole::Tool,
                    content: result.content,
                    name: Some(call.name),
                });
            }
        }

        self.pipeline.finalize(&mut state).await?;

        let record = MemoryRecord::new(
            MemoryTier::Working,
            input.session_id,
            input.agent_id,
            state.assistant_message.clone(),
        );
        self.memory.write(record).await?;

        self.events
            .publish(
                EventEnvelope::new(
                    EventKind::MemoryWritten,
                    input.tenant.tenant_id,
                    serde_json::json!({ "tier": "working" }),
                )
                .with_session(input.session_id)
                .with_agent(input.agent_id)
                .with_run(run_id),
            )
            .await?;

        let summary = state.session_summary.clone();
        if !summary.is_empty() {
            self.memory
                .compress_session(input.session_id, summary)
                .await?;
        }

        info!(tool_rounds, "agent run completed");

        self.events
            .publish(
                EventEnvelope::new(
                    EventKind::AgentRunCompleted,
                    input.tenant.tenant_id,
                    serde_json::json!({ "tool_rounds": tool_rounds }),
                )
                .with_session(input.session_id)
                .with_agent(input.agent_id)
                .with_run(run_id),
            )
            .await?;

        Ok(AgentRunOutput {
            assistant_message: state.assistant_message,
            messages: state.messages,
            tool_rounds,
            prompt_tokens: total_prompt,
            completion_tokens: total_completion,
        })
    }

    pub async fn load_working_memory(
        &self,
        session_id: clawrs_core::SessionId,
        limit: usize,
    ) -> ClawrsResult<Vec<MemoryRecord>> {
        self.memory
            .query(MemoryQuery::for_session(session_id, limit))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clawrs_core::{TenantContext, TenantId, WorkspaceId};
    use clawrs_events::InMemoryEventBus;
    use clawrs_llm::MockProvider;
    use clawrs_memory::InMemoryMemoryStore;
    use clawrs_tools::EchoTool;
    use std::sync::Arc;

    #[tokio::test]
    async fn runner_completes_without_tools() {
        let events = Arc::new(InMemoryEventBus::default());
        let memory = Arc::new(InMemoryMemoryStore::default());
        let provider = Arc::new(MockProvider::new("mock"));
        let tools = Arc::new(ToolRegistry::new());
        tools.register(Arc::new(EchoTool));

        let runner = AgentRunner::new(events, memory.clone(), provider, tools);
        let tenant = TenantContext::system(TenantId::new_v4(), WorkspaceId::new_v4());
        let out = runner
            .run(AgentRunInput {
                tenant,
                agent_id: clawrs_core::AgentId::new_v4(),
                session_id: clawrs_core::SessionId::new_v4(),
                kind: crate::kind::AgentKind::General,
                prompt_mode: PromptMode::Full,
                model: "mock-model".into(),
                user_message: "hello".into(),
                max_tool_rounds: 3,
                system_prompt: None,
                prior_messages: vec![],
            })
            .await
            .unwrap();

        assert!(out.assistant_message.contains("hello"));
    }
}
