use crate::models::{AgentRow, MessageRow, SessionRow, WorkspaceBootstrap};
use crate::pool::{StorePool, StoreResult};
use chrono::Utc;
use clawrs_agent::AgentKind;
use clawrs_core::{AgentId, SessionId, TenantId, WorkspaceId};
use clawrs_llm::{ChatMessage, MessageRole};
use sqlx::Row;
use uuid::Uuid;

pub struct StoreRepository<'a> {
    pool: &'a StorePool,
}

impl<'a> StoreRepository<'a> {
    pub fn new(pool: &'a StorePool) -> Self {
        Self { pool }
    }

    pub async fn ensure_bootstrap(&self) -> StoreResult<WorkspaceBootstrap> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tenants")
            .fetch_one(self.pool.inner())
            .await?;
        if count > 0 {
            return self.load_bootstrap().await;
        }

        let tenant_id = TenantId::new_v4();
        let workspace_id = WorkspaceId::new_v4();
        let agent_id = AgentId::new_v4();
        let now = Utc::now().to_rfc3339();

        sqlx::query("INSERT INTO tenants (id, name, created_at) VALUES (?, ?, ?)")
            .bind(tenant_id.to_string())
            .bind("Default")
            .bind(&now)
            .execute(self.pool.inner())
            .await?;

        sqlx::query("INSERT INTO workspaces (id, tenant_id, name, created_at) VALUES (?, ?, ?, ?)")
            .bind(workspace_id.to_string())
            .bind(tenant_id.to_string())
            .bind("Main")
            .bind(&now)
            .execute(self.pool.inner())
            .await?;

        sqlx::query(
            "INSERT INTO agents (id, workspace_id, name, kind, model, system_prompt, description, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(agent_id.to_string())
        .bind(workspace_id.to_string())
        .bind("Claw")
        .bind("general")
        .bind("gpt-4o-mini")
        .bind("You are Claw, a precise and capable agent on the ClawRS platform. Be direct, helpful, and safe with tools.")
        .bind("Default workspace agent")
        .bind(&now)
        .bind(&now)
        .execute(self.pool.inner())
        .await?;

        Ok(WorkspaceBootstrap {
            tenant_id,
            workspace_id,
            default_agent_id: agent_id,
        })
    }

    async fn load_bootstrap(&self) -> StoreResult<WorkspaceBootstrap> {
        let tenant: String = sqlx::query_scalar("SELECT id FROM tenants LIMIT 1")
            .fetch_one(self.pool.inner())
            .await?;
        let workspace: String = sqlx::query_scalar("SELECT id FROM workspaces LIMIT 1")
            .fetch_one(self.pool.inner())
            .await?;
        let agent: String = sqlx::query_scalar("SELECT id FROM agents LIMIT 1")
            .fetch_one(self.pool.inner())
            .await?;

        Ok(WorkspaceBootstrap {
            tenant_id: TenantId::from_str(&tenant).unwrap(),
            workspace_id: WorkspaceId::from_str(&workspace).unwrap(),
            default_agent_id: AgentId::from_str(&agent).unwrap(),
        })
    }

    pub async fn list_agents(&self, workspace_id: WorkspaceId) -> StoreResult<Vec<AgentRow>> {
        let rows = sqlx::query(
            "SELECT id, workspace_id, name, kind, model, system_prompt, description, created_at, updated_at
             FROM agents WHERE workspace_id = ? ORDER BY name",
        )
        .bind(workspace_id.to_string())
        .fetch_all(self.pool.inner())
        .await?;

        rows.into_iter().map(map_agent).collect()
    }

    pub async fn get_agent(&self, id: AgentId) -> StoreResult<Option<AgentRow>> {
        let row = sqlx::query(
            "SELECT id, workspace_id, name, kind, model, system_prompt, description, created_at, updated_at
             FROM agents WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(self.pool.inner())
        .await?;
        row.map(map_agent).transpose()
    }

    pub async fn create_agent(
        &self,
        workspace_id: WorkspaceId,
        name: &str,
        kind: AgentKind,
        model: &str,
        system_prompt: &str,
        description: &str,
    ) -> StoreResult<AgentRow> {
        let id = AgentId::new_v4();
        let now = Utc::now();
        let kind_str = kind_to_str(kind);
        sqlx::query(
            "INSERT INTO agents (id, workspace_id, name, kind, model, system_prompt, description, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(id.to_string())
        .bind(workspace_id.to_string())
        .bind(name)
        .bind(kind_str)
        .bind(model)
        .bind(system_prompt)
        .bind(description)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(self.pool.inner())
        .await?;

        Ok(AgentRow {
            id,
            workspace_id,
            name: name.into(),
            kind,
            model: model.into(),
            system_prompt: system_prompt.into(),
            description: description.into(),
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_session(&self, id: SessionId) -> StoreResult<Option<SessionRow>> {
        let row = sqlx::query(
            "SELECT id, agent_id, title, created_at, updated_at FROM sessions WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(self.pool.inner())
        .await?;
        row.map(map_session).transpose()
    }

    pub async fn list_sessions(&self, agent_id: AgentId) -> StoreResult<Vec<SessionRow>> {
        let rows = sqlx::query(
            "SELECT id, agent_id, title, created_at, updated_at FROM sessions WHERE agent_id = ? ORDER BY updated_at DESC",
        )
        .bind(agent_id.to_string())
        .fetch_all(self.pool.inner())
        .await?;
        rows.into_iter().map(map_session).collect()
    }

    pub async fn create_session(&self, agent_id: AgentId, title: &str) -> StoreResult<SessionRow> {
        let id = SessionId::new_v4();
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO sessions (id, agent_id, title, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(id.to_string())
        .bind(agent_id.to_string())
        .bind(title)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(self.pool.inner())
        .await?;
        Ok(SessionRow {
            id,
            agent_id,
            title: title.into(),
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn touch_session(&self, session_id: SessionId) -> StoreResult<()> {
        sqlx::query("UPDATE sessions SET updated_at = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339())
            .bind(session_id.to_string())
            .execute(self.pool.inner())
            .await?;
        Ok(())
    }

    pub async fn update_session_title(&self, session_id: SessionId, title: &str) -> StoreResult<()> {
        sqlx::query("UPDATE sessions SET title = ?, updated_at = ? WHERE id = ?")
            .bind(title)
            .bind(Utc::now().to_rfc3339())
            .bind(session_id.to_string())
            .execute(self.pool.inner())
            .await?;
        Ok(())
    }

    pub async fn list_messages(&self, session_id: SessionId) -> StoreResult<Vec<MessageRow>> {
        let rows = sqlx::query(
            "SELECT id, session_id, role, content, created_at FROM messages WHERE session_id = ? ORDER BY created_at ASC",
        )
        .bind(session_id.to_string())
        .fetch_all(self.pool.inner())
        .await?;
        rows.into_iter().map(map_message).collect()
    }

    pub async fn append_message(
        &self,
        session_id: SessionId,
        role: MessageRole,
        content: &str,
    ) -> StoreResult<MessageRow> {
        let id = Uuid::now_v7();
        let now = Utc::now();
        let role_str = role_to_str(role);
        sqlx::query(
            "INSERT INTO messages (id, session_id, role, content, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(id.to_string())
        .bind(session_id.to_string())
        .bind(role_str)
        .bind(content)
        .bind(now.to_rfc3339())
        .execute(self.pool.inner())
        .await?;
        self.touch_session(session_id).await?;
        Ok(MessageRow {
            id,
            session_id,
            role: role_str.into(),
            content: content.into(),
            created_at: now,
        })
    }

    pub async fn messages_as_chat(&self, session_id: SessionId) -> StoreResult<Vec<ChatMessage>> {
        let rows = self.list_messages(session_id).await?;
        Ok(rows.into_iter().filter_map(row_to_chat).collect())
    }

    pub async fn stats(&self) -> StoreResult<PlatformStats> {
        let agents: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agents")
            .fetch_one(self.pool.inner())
            .await?;
        let sessions: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sessions")
            .fetch_one(self.pool.inner())
            .await?;
        let messages: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages")
            .fetch_one(self.pool.inner())
            .await?;
        Ok(PlatformStats {
            agents,
            sessions,
            messages,
        })
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct PlatformStats {
    pub agents: i64,
    pub sessions: i64,
    pub messages: i64,
}

use std::str::FromStr;

fn map_agent(row: sqlx::sqlite::SqliteRow) -> StoreResult<AgentRow> {
    let id: String = row.get("id");
    let workspace_id: String = row.get("workspace_id");
    let kind: String = row.get("kind");
    Ok(AgentRow {
        id: AgentId::from_str(&id).unwrap(),
        workspace_id: WorkspaceId::from_str(&workspace_id).unwrap(),
        name: row.get("name"),
        kind: str_to_kind(&kind),
        model: row.get("model"),
        system_prompt: row.get("system_prompt"),
        description: row.get("description"),
        created_at: parse_ts(row.get::<String, _>("created_at")),
        updated_at: parse_ts(row.get::<String, _>("updated_at")),
    })
}

fn map_session(row: sqlx::sqlite::SqliteRow) -> StoreResult<SessionRow> {
    Ok(SessionRow {
        id: SessionId::from_str(&row.get::<String, _>("id")).unwrap(),
        agent_id: AgentId::from_str(&row.get::<String, _>("agent_id")).unwrap(),
        title: row.get("title"),
        created_at: parse_ts(row.get("created_at")),
        updated_at: parse_ts(row.get("updated_at")),
    })
}

fn map_message(row: sqlx::sqlite::SqliteRow) -> StoreResult<MessageRow> {
    Ok(MessageRow {
        id: Uuid::parse_str(&row.get::<String, _>("id")).unwrap(),
        session_id: SessionId::from_str(&row.get::<String, _>("session_id")).unwrap(),
        role: row.get("role"),
        content: row.get("content"),
        created_at: parse_ts(row.get("created_at")),
    })
}

fn parse_ts(s: String) -> chrono::DateTime<Utc> {
    chrono::DateTime::parse_from_rfc3339(&s)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn kind_to_str(kind: AgentKind) -> &'static str {
    match kind {
        AgentKind::General => "general",
        AgentKind::Worker => "worker",
        AgentKind::Planner => "planner",
        AgentKind::Critic => "critic",
        AgentKind::Reflection => "reflection",
        AgentKind::Background => "background",
        AgentKind::SubAgent => "sub_agent",
    }
}

fn str_to_kind(s: &str) -> AgentKind {
    match s {
        "worker" => AgentKind::Worker,
        "planner" => AgentKind::Planner,
        "critic" => AgentKind::Critic,
        "reflection" => AgentKind::Reflection,
        "background" => AgentKind::Background,
        "sub_agent" => AgentKind::SubAgent,
        _ => AgentKind::General,
    }
}

fn role_to_str(role: MessageRole) -> &'static str {
    match role {
        MessageRole::System => "system",
        MessageRole::User => "user",
        MessageRole::Assistant => "assistant",
        MessageRole::Tool => "tool",
    }
}

fn row_to_chat(row: MessageRow) -> Option<ChatMessage> {
    let role = match row.role.as_str() {
        "system" => MessageRole::System,
        "user" => MessageRole::User,
        "assistant" => MessageRole::Assistant,
        "tool" => MessageRole::Tool,
        _ => return None,
    };
    Some(ChatMessage {
        role,
        content: row.content,
        name: None,
    })
}
