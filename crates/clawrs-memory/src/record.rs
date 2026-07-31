use crate::tier::MemoryTier;
use chrono::{DateTime, Utc};
use clawrs_core::{AgentId, SessionId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MemoryRecord {
    pub id: Uuid,
    pub tier: MemoryTier,
    pub session_id: SessionId,
    pub agent_id: AgentId,
    pub content: String,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub importance: f32,
}

impl MemoryRecord {
    pub fn new(
        tier: MemoryTier,
        session_id: SessionId,
        agent_id: AgentId,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            tier,
            session_id,
            agent_id,
            content: content.into(),
            metadata: serde_json::Value::Null,
            created_at: Utc::now(),
            importance: 0.5,
        }
    }
}
