use crate::context::ToolContext;
use crate::tool::{Tool, ToolMetadata, ToolResult};
use clawrs_core::{ClawrsError, ClawrsResult};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

pub struct ToolRegistry {
    tools: RwLock<HashMap<String, Arc<dyn Tool>>>,
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, tool: Arc<dyn Tool>) {
        let name = tool.metadata().name.clone();
        self.tools.write().insert(name, tool);
    }

    pub fn list_metadata(&self) -> Vec<ToolMetadata> {
        self.tools
            .read()
            .values()
            .map(|t| t.metadata())
            .collect()
    }

    pub async fn invoke(
        &self,
        name: &str,
        ctx: &ToolContext,
        arguments: serde_json::Value,
    ) -> ClawrsResult<ToolResult> {
        let tool = self.tools.read().get(name).cloned().ok_or_else(|| {
            ClawrsError::NotFound {
                resource: "tool",
                id: name.to_string(),
            }
        })?;
        if ctx.dry_run {
            return Ok(ToolResult {
                content: format!("dry-run: would invoke `{name}`"),
                structured: Some(arguments),
            });
        }
        tool.invoke(ctx, arguments).await
    }
}
