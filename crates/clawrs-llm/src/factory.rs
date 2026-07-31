//! Builds a concrete LLM provider from configuration.

use crate::mock::MockProvider;
use crate::openai_compatible::{OpenAiCompatibleConfig, OpenAiCompatibleProvider};
use crate::provider::{LlmProvider, ProviderId};
use async_trait::async_trait;
use clawrs_core::{ClawrsError, ClawrsResult};
use std::sync::Arc;

pub enum AnyProvider {
    Mock(MockProvider),
    OpenAi(OpenAiCompatibleProvider),
}

#[async_trait]
impl LlmProvider for AnyProvider {
    fn id(&self) -> ProviderId {
        match self {
            Self::Mock(p) => p.id(),
            Self::OpenAi(p) => p.id(),
        }
    }

    async fn complete(
        &self,
        request: crate::request::CompletionRequest,
    ) -> ClawrsResult<crate::response::CompletionResponse> {
        match self {
            Self::Mock(p) => p.complete(request).await,
            Self::OpenAi(p) => p.complete(request).await,
        }
    }

    async fn stream(
        &self,
        request: crate::request::CompletionRequest,
    ) -> ClawrsResult<crate::provider::CompletionStream> {
        match self {
            Self::Mock(p) => p.stream(request).await,
            Self::OpenAi(p) => p.stream(request).await,
        }
    }
}

pub struct LlmFactory;

pub struct DynProvider(pub Arc<dyn LlmProvider>);

#[async_trait]
impl LlmProvider for DynProvider {
    fn id(&self) -> ProviderId {
        self.0.id()
    }

    async fn complete(
        &self,
        request: crate::request::CompletionRequest,
    ) -> ClawrsResult<crate::response::CompletionResponse> {
        self.0.complete(request).await
    }

    async fn stream(
        &self,
        request: crate::request::CompletionRequest,
    ) -> ClawrsResult<crate::provider::CompletionStream> {
        self.0.stream(request).await
    }
}

impl LlmFactory {
    pub fn from_config(
        provider: clawrs_config::LlmProviderKind,
        api_key: Option<String>,
        base_url: String,
        default_model: String,
    ) -> ClawrsResult<DynProvider> {
        let inner: Arc<dyn LlmProvider> = match provider {
            clawrs_config::LlmProviderKind::Mock => {
                Arc::new(AnyProvider::Mock(MockProvider::new("mock")))
            }
            clawrs_config::LlmProviderKind::OpenAiCompatible => {
                let key = api_key.ok_or_else(|| {
                    ClawrsError::validation("OpenAI-compatible provider requires an API key")
                })?;
                let config = OpenAiCompatibleConfig {
                    base_url,
                    api_key: Some(key),
                    provider_id: ProviderId::new("openai-compatible"),
                    default_model,
                };
                Arc::new(AnyProvider::OpenAi(
                    OpenAiCompatibleProvider::new(config)?,
                ))
            }
        };
        Ok(DynProvider(inner))
    }
}
