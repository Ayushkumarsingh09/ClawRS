use thiserror::Error;

/// Unified domain and application error type.
#[derive(Debug, Error)]
pub enum ClawrsError {
    #[error("not found: {resource} `{id}`")]
    NotFound { resource: &'static str, id: String },

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("rate limited: retry after {retry_after_ms}ms")]
    RateLimited { retry_after_ms: u64 },

    #[error("provider error ({provider}): {message}")]
    Provider {
        provider: String,
        message: String,
    },

    #[error("tool execution failed ({tool}): {message}")]
    Tool {
        tool: String,
        message: String,
    },

    #[error("pipeline stage `{stage}` failed: {message}")]
    PipelineStage { stage: String, message: String },

    #[error("internal error: {0}")]
    Internal(String),
}

pub type ClawrsResult<T> = Result<T, ClawrsError>;

impl ClawrsError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }
}
