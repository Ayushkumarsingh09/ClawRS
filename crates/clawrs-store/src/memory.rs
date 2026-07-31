use async_trait::async_trait;
use chrono::Utc;
use clawrs_core::{AgentId, ClawrsResult, SessionId};
use clawrs_memory::{MemoryQuery, MemoryRecord, MemoryStore, MemoryTier};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SqliteMemoryStore {
    pool: SqlitePool,
}

impl SqliteMemoryStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MemoryStore for SqliteMemoryStore {
    async fn write(&self, record: MemoryRecord) -> ClawrsResult<()> {
        sqlx::query(
            "INSERT INTO memory_records (id, session_id, agent_id, tier, content, importance, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(record.id.to_string())
        .bind(record.session_id.to_string())
        .bind(record.agent_id.to_string())
        .bind(match record.tier {
            MemoryTier::Working => "working",
            MemoryTier::Episodic => "episodic",
            MemoryTier::Semantic => "semantic",
        })
        .bind(record.content)
        .bind(record.importance)
        .bind(record.created_at.to_rfc3339())
        .execute(&self.pool)
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?;
        Ok(())
    }

    async fn query(&self, query: MemoryQuery) -> ClawrsResult<Vec<MemoryRecord>> {
        let session = query
            .session_id
            .map(|s| s.to_string())
            .unwrap_or_default();
        let rows = if query.session_id.is_some() {
            sqlx::query_as::<_, (String, String, String, String, String, f32, String)>(
                "SELECT id, session_id, agent_id, tier, content, importance, created_at
                 FROM memory_records WHERE session_id = ? ORDER BY created_at DESC LIMIT ?",
            )
            .bind(session)
            .bind(query.limit as i64)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as::<_, (String, String, String, String, String, f32, String)>(
                "SELECT id, session_id, agent_id, tier, content, importance, created_at
                 FROM memory_records ORDER BY created_at DESC LIMIT ?",
            )
            .bind(query.limit as i64)
            .fetch_all(&self.pool)
            .await
        }
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?;

        Ok(rows
            .into_iter()
            .filter_map(|r| {
                Some(MemoryRecord {
                    id: Uuid::parse_str(&r.0).ok()?,
                    session_id: SessionId::from_str(&r.1).ok()?,
                    agent_id: AgentId::from_str(&r.2).ok()?,
                    tier: MemoryTier::Working,
                    content: r.4,
                    metadata: serde_json::Value::Null,
                    created_at: chrono::DateTime::parse_from_rfc3339(&r.6)
                        .ok()?
                        .with_timezone(&Utc),
                    importance: r.5,
                })
            })
            .collect())
    }

    async fn compress_session(&self, session_id: SessionId, summary: String) -> ClawrsResult<()> {
        let record = MemoryRecord::new(
            MemoryTier::Episodic,
            session_id,
            AgentId::new_v4(),
            summary,
        );
        self.write(record).await
    }
}

use std::str::FromStr;
