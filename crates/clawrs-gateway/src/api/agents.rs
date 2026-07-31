use axum::{extract::State, Json};
use clawrs_agent::AgentKind;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::state::AppState;

#[derive(Serialize)]
pub struct AgentsResponse {
    pub agents: Vec<clawrs_store::models::AgentRow>,
}

pub async fn list(State(state): State<AppState>) -> Result<Json<AgentsResponse>, ApiError> {
    let agents = state
        .repo()
        .list_agents(state.bootstrap.workspace_id)
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?;
    Ok(Json(AgentsResponse { agents }))
}

#[derive(Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    #[serde(default = "default_kind")]
    pub kind: AgentKind,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub system_prompt: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

fn default_kind() -> AgentKind {
    AgentKind::General
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateAgentRequest>,
) -> Result<Json<clawrs_store::models::AgentRow>, ApiError> {
    let model = body
        .model
        .unwrap_or_else(|| state.default_model.clone());
    let agent = state
        .repo()
        .create_agent(
            state.bootstrap.workspace_id,
            &body.name,
            body.kind,
            &model,
            body.system_prompt
                .as_deref()
                .unwrap_or("You are a helpful ClawRS agent."),
            body.description.as_deref().unwrap_or(""),
        )
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?;
    Ok(Json(agent))
}

pub async fn get_one(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<clawrs_core::AgentId>,
) -> Result<Json<clawrs_store::models::AgentRow>, ApiError> {
    let agent = state
        .repo()
        .get_agent(id)
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?
        .ok_or_else(|| clawrs_core::ClawrsError::NotFound {
            resource: "agent",
            id: id.to_string(),
        })?;
    Ok(Json(agent))
}
