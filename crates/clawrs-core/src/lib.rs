//! Core domain primitives for the ClawRS platform.

pub mod error;
pub mod ids;
pub mod tenant;
pub mod version;

pub use error::{ClawrsError, ClawrsResult};
pub use ids::{AgentId, RunId, SessionId, TenantId, ToolCallId, UserId};
pub use tenant::{TenantContext, WorkspaceId};
pub use version::{BUILD_PROFILE, VERSION};
