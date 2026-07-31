use clawrs_core::{AgentId, SessionId, TenantContext};

#[derive(Clone, Debug)]
pub struct ToolContext {
    pub tenant: TenantContext,
    pub session_id: SessionId,
    pub agent_id: AgentId,
    pub dry_run: bool,
}
