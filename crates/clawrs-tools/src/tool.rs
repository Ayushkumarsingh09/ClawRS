use crate::context::ToolContext;
use async_trait::async_trait;
use clawrs_core::ClawrsResult;
use clawrs_llm::ToolDefinition;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub name: String,
    pub description: String,
    pub parameters_schema: serde_json::Value,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolResult {
    pub content: String,
    pub structured: Option<serde_json::Value>,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn metadata(&self) -> ToolMetadata;

    async fn invoke(&self, ctx: &ToolContext, arguments: serde_json::Value)
        -> ClawrsResult<ToolResult>;
}

impl ToolMetadata {
    pub fn to_llm_definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: self.name.clone(),
            description: self.description.clone(),
            parameters_schema: self.parameters_schema.clone(),
        }
    }
}
