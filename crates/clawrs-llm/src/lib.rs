//! Provider-agnostic LLM interface with streaming and structured tool calls.

pub mod message;
pub mod mock;
pub mod provider;
pub mod request;
pub mod response;

#[cfg(feature = "openai-compatible")]
pub mod openai_compatible;
pub mod factory;

pub use factory::{AnyProvider, DynProvider, LlmFactory};

pub use message::{ChatMessage, MessageRole, ToolDefinition};
pub use mock::MockProvider;
pub use provider::{LlmProvider, ProviderId};
pub use request::{CompletionRequest, RequestOptions, ResponseFormat};
pub use response::{CompletionResponse, FinishReason, StreamChunk, TokenUsage, ToolCallRequest};
