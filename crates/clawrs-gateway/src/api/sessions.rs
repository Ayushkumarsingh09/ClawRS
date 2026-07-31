use axum::{extract::State, Json};
use clawrs_core::AgentId;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Serialize)]
pub struct SessionsResponse {
    pub sessions: Vec<clawrs_store::models::SessionRow>,
}

#[derive(Deserialize)]
pub struct ListSessionsQuery {
    pub agent_id: AgentId,
}

pub async fn list(
    State(state): State<AppState>,
    axum::extract::Query(q): axum::extract::Query<ListSessionsQuery>,
) -> Result<Json<SessionsResponse>, ApiError> {
    let sessions = state
        .repo()
        .list_sessions(q.agent_id)
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?;
    Ok(Json(SessionsResponse { sessions }))
}

#[derive(Deserialize)]
pub struct CreateSessionRequest {
    pub agent_id: AgentId,
    #[serde(default = "default_title")]
    pub title: String,
}

fn default_title() -> String {
    "New chat".into()
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateSessionRequest>,
) -> Result<Json<clawrs_store::models::SessionRow>, ApiError> {
    let session = state
        .repo()
        .create_session(body.agent_id, &body.title)
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?;
    Ok(Json(session))
}

pub async fn messages(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<clawrs_core::SessionId>,
) -> Result<Json<MessagesResponse>, ApiError> {
    let messages = state
        .repo()
        .list_messages(id)
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?;
    Ok(Json(MessagesResponse { messages }))
}

#[derive(Serialize)]
pub struct MessagesResponse {
    pub messages: Vec<clawrs_store::models::MessageRow>,
}
