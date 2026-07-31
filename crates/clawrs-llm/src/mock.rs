use crate::provider::{LlmProvider, ProviderId};
use crate::request::CompletionRequest;
use crate::response::{CompletionResponse, FinishReason, TokenUsage};
use async_trait::async_trait;
use clawrs_core::ClawrsResult;
use std::sync::atomic::{AtomicU64, Ordering};

/// Deterministic provider for unit tests and offline development.
pub struct MockProvider {
    id: ProviderId,
    calls: AtomicU64,
    reply_prefix: String,
}

impl MockProvider {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: ProviderId::new(id),
            calls: AtomicU64::new(0),
            reply_prefix: "mock:".into(),
        }
    }

    pub fn call_count(&self) -> u64 {
        self.calls.load(Ordering::Relaxed)
    }
}

#[async_trait]
impl LlmProvider for MockProvider {
    fn id(&self) -> ProviderId {
        self.id.clone()
    }

    async fn complete(&self, request: CompletionRequest) -> ClawrsResult<CompletionResponse> {
        self.calls.fetch_add(1, Ordering::Relaxed);
        let last_user = request
            .messages
            .iter()
            .rev()
            .find(|m| matches!(m.role, crate::message::MessageRole::User))
            .map(|m| m.content.as_str())
            .unwrap_or("");
        Ok(CompletionResponse {
            content: format!("{}{}", self.reply_prefix, last_user),
            tool_calls: Vec::new(),
            finish_reason: FinishReason::Stop,
            usage: TokenUsage {
                prompt_tokens: 10,
                completion_tokens: 5,
                total_tokens: 15,
            },
            model: request.model,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::CompletionRequest;

    #[tokio::test]
    async fn mock_echoes_user() {
        let provider = MockProvider::new("mock");
        let resp = provider
            .complete(CompletionRequest::single_turn("test-model", "ping"))
            .await
            .unwrap();
        assert_eq!(resp.content, "mock:ping");
        assert_eq!(provider.call_count(), 1);
    }
}
