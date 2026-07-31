use crate::context::ToolContext;
use crate::tool::{Tool, ToolMetadata, ToolResult};
use async_trait::async_trait;
use clawrs_core::ClawrsResult;
use serde_json::json;

pub struct EchoTool;

#[async_trait]
impl Tool for EchoTool {
    fn metadata(&self) -> ToolMetadata {
        ToolMetadata {
            name: "echo".into(),
            description: "Returns the JSON arguments payload as formatted text.".into(),
            parameters_schema: json!({
                "type": "object",
                "properties": {
                    "message": { "type": "string" }
                }
            }),
        }
    }

    async fn invoke(
        &self,
        _ctx: &ToolContext,
        arguments: serde_json::Value,
    ) -> ClawrsResult<ToolResult> {
        Ok(ToolResult {
            content: arguments.to_string(),
            structured: Some(arguments),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clawrs_core::{AgentId, SessionId, TenantContext, TenantId, WorkspaceId};
    use std::sync::Arc;

    #[tokio::test]
    async fn echo_returns_arguments() {
        let tool = EchoTool;
        let ctx = ToolContext {
            tenant: TenantContext::system(TenantId::new_v4(), WorkspaceId::new_v4()),
            session_id: SessionId::new_v4(),
            agent_id: AgentId::new_v4(),
            dry_run: false,
        };
        let result = tool
            .invoke(&ctx, json!({"message": "hi"}))
            .await
            .unwrap();
        assert!(result.content.contains("hi"));
    }

    #[test]
    fn registry_lists_tools() {
        let registry = crate::registry::ToolRegistry::new();
        registry.register(Arc::new(EchoTool));
        assert_eq!(registry.list_metadata().len(), 1);
    }
}
