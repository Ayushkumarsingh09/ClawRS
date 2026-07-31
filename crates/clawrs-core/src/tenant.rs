use crate::ids::{AgentId, TenantId, UserId};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Workspace groups agents and knowledge within a tenant.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WorkspaceId(Uuid);

impl WorkspaceId {
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Display for WorkspaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for WorkspaceId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

/// Request-scoped tenant identity propagated through the platform.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TenantContext {
    pub tenant_id: TenantId,
    pub workspace_id: WorkspaceId,
    pub user_id: Option<UserId>,
    pub acting_agent_id: Option<AgentId>,
}

impl TenantContext {
    pub fn system(tenant_id: TenantId, workspace_id: WorkspaceId) -> Self {
        Self {
            tenant_id,
            workspace_id,
            user_id: None,
            acting_agent_id: None,
        }
    }
}
