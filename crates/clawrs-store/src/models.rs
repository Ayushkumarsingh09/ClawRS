use chrono::{DateTime, Utc};
use clawrs_agent::AgentKind;
use clawrs_core::{AgentId, SessionId, TenantId, WorkspaceId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentRow {
    pub id: AgentId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub kind: AgentKind,
    pub model: String,
    pub system_prompt: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionRow {
    pub id: SessionId,
    pub agent_id: AgentId,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageRow {
    pub id: uuid::Uuid,
    pub session_id: SessionId,
    pub role: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkspaceBootstrap {
    pub tenant_id: TenantId,
    pub workspace_id: WorkspaceId,
    pub default_agent_id: AgentId,
}
