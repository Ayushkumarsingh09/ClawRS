use crate::record::MemoryRecord;
use crate::tier::MemoryTier;
use async_trait::async_trait;
use clawrs_core::{AgentId, ClawrsResult, SessionId};
use parking_lot::RwLock;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct MemoryQuery {
    pub session_id: Option<SessionId>,
    pub agent_id: Option<AgentId>,
    pub tier: Option<MemoryTier>,
    pub limit: usize,
    pub min_importance: Option<f32>,
}

impl MemoryQuery {
    pub fn for_session(session_id: SessionId, limit: usize) -> Self {
        Self {
            session_id: Some(session_id),
            agent_id: None,
            tier: None,
            limit,
            min_importance: None,
        }
    }
}

#[async_trait]
pub trait MemoryStore: Send + Sync {
    async fn write(&self, record: MemoryRecord) -> ClawrsResult<()>;
    async fn query(&self, query: MemoryQuery) -> ClawrsResult<Vec<MemoryRecord>>;
    async fn compress_session(&self, session_id: SessionId, summary: String) -> ClawrsResult<()>;
}


pub struct InMemoryMemoryStore {
    records: RwLock<Vec<MemoryRecord>>,
    summaries: RwLock<HashMap<SessionId, String>>,
}

impl Default for InMemoryMemoryStore {
    fn default() -> Self {
        Self {
            records: RwLock::new(Vec::new()),
            summaries: RwLock::new(HashMap::new()),
        }
    }
}

impl InMemoryMemoryStore {
    pub fn session_summary(&self, session_id: SessionId) -> Option<String> {
        self.summaries.read().get(&session_id).cloned()
    }
}

#[async_trait]
impl MemoryStore for InMemoryMemoryStore {
    async fn write(&self, record: MemoryRecord) -> ClawrsResult<()> {
        self.records.write().push(record);
        Ok(())
    }

    async fn query(&self, query: MemoryQuery) -> ClawrsResult<Vec<MemoryRecord>> {
        let records = self.records.read();
        let mut out: Vec<_> = records
            .iter()
            .filter(|r| query.session_id.is_none_or(|s| r.session_id == s))
            .filter(|r| query.agent_id.is_none_or(|a| r.agent_id == a))
            .filter(|r| query.tier.is_none_or(|t| r.tier == t))
            .filter(|r| query.min_importance.is_none_or(|m| r.importance >= m))
            .cloned()
            .collect();
        out.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        out.truncate(query.limit);
        Ok(out)
    }

    async fn compress_session(&self, session_id: SessionId, summary: String) -> ClawrsResult<()> {
        self.summaries.write().insert(session_id, summary);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clawrs_core::AgentId;

    #[tokio::test]
    async fn query_filters_by_session() {
        let store = InMemoryMemoryStore::default();
        let session = SessionId::new_v4();
        let agent = AgentId::new_v4();
        store
            .write(MemoryRecord::new(
                MemoryTier::Working,
                session,
                agent,
                "hello",
            ))
            .await
            .unwrap();
        let other_session = SessionId::new_v4();
        store
            .write(MemoryRecord::new(
                MemoryTier::Working,
                other_session,
                agent,
                "other",
            ))
            .await
            .unwrap();
        let results = store
            .query(MemoryQuery::for_session(session, 10))
            .await
            .unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].content, "hello");
    }
}
