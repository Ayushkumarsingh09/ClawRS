use crate::request::CompletionRequest;
use crate::response::{CompletionResponse, StreamChunk};
use async_trait::async_trait;
use clawrs_core::ClawrsResult;
use futures::Stream;
use std::pin::Pin;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ProviderId(pub String);

impl ProviderId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

pub type CompletionStream =
    Pin<Box<dyn Stream<Item = ClawrsResult<StreamChunk>> + Send + 'static>>;

#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn id(&self) -> ProviderId;

    async fn complete(&self, request: CompletionRequest) -> ClawrsResult<CompletionResponse>;

    async fn stream(&self, request: CompletionRequest) -> ClawrsResult<CompletionStream> {
        let response = self.complete(request).await?;
        let chunk = StreamChunk {
            delta: response.content.clone(),
            finish_reason: Some(response.finish_reason),
        };
        let stream = futures::stream::once(async move { Ok(chunk) });
        Ok(Box::pin(stream))
    }
}
