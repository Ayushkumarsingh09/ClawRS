//! Plugin-first tool execution framework.

pub mod builtin;
pub mod context;
pub mod registry;
pub mod tool;

pub use builtin::EchoTool;
pub use context::ToolContext;
pub use registry::ToolRegistry;
pub use tool::{Tool, ToolMetadata, ToolResult};
