use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

macro_rules! id_type {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
        #[serde(transparent)]
        pub struct $name(Uuid);

        impl $name {
            pub fn new_v4() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn new_v7() -> Self {
                Self(Uuid::now_v7())
            }

            /// Stable ID derived from an external provider identifier (e.g. OpenAI tool call id).
            pub fn from_external(s: &str) -> Self {
                Self(Uuid::new_v5(&Uuid::NAMESPACE_URL, s.as_bytes()))
            }

            pub fn from_uuid(uuid: Uuid) -> Self {
                Self(uuid)
            }

            pub fn as_uuid(&self) -> &Uuid {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }

        impl FromStr for $name {
            type Err = uuid::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(Uuid::parse_str(s)?))
            }
        }
    };
}

id_type!(TenantId, "Isolation boundary for multi-tenant deployments.");
id_type!(UserId, "Human or service principal within a tenant.");
id_type!(AgentId, "Unique agent definition identifier.");
id_type!(SessionId, "Conversation or run session identifier.");
id_type!(RunId, "Single agent execution (turn) identifier.");
id_type!(ToolCallId, "Identifier for an LLM-requested tool invocation.");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn agent_id_roundtrip_json() {
        let id = AgentId::new_v4();
        let json = serde_json::to_string(&id).unwrap();
        let parsed: AgentId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, parsed);
    }
}
