use chrono::{DateTime, Utc};
use clawrs_core::{AgentId, RunId, SessionId, TenantId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// High-level event classification for routing and observability.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    AgentRunStarted,
    AgentRunCompleted,
    AgentRunFailed,
    ToolInvoked,
    ToolCompleted,
    MemoryWritten,
    LlmRequestStarted,
    LlmRequestCompleted,
    Custom(String),
}

/// Wrapper for all domain events flowing through the platform.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: Uuid,
    pub kind: EventKind,
    pub occurred_at: DateTime<Utc>,
    pub tenant_id: TenantId,
    pub session_id: Option<SessionId>,
    pub agent_id: Option<AgentId>,
    pub run_id: Option<RunId>,
    pub payload: serde_json::Value,
}

impl EventEnvelope {
    pub fn new(kind: EventKind, tenant_id: TenantId, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::now_v7(),
            kind,
            occurred_at: Utc::now(),
            tenant_id,
            session_id: None,
            agent_id: None,
            run_id: None,
            payload,
        }
    }

    pub fn with_session(mut self, session_id: SessionId) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn with_agent(mut self, agent_id: AgentId) -> Self {
        self.agent_id = Some(agent_id);
        self
    }

    pub fn with_run(mut self, run_id: RunId) -> Self {
        self.run_id = Some(run_id);
        self
    }
}
