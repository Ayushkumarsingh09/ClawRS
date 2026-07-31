use axum::{extract::State, Json};
use clawrs_core::VERSION;
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: VERSION,
    })
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub version: &'static str,
    pub provider: String,
    pub default_model: String,
    pub stats: clawrs_store::PlatformStats,
}

pub async fn status(State(state): State<AppState>) -> Result<Json<StatusResponse>, crate::error::ApiError> {
    let stats = state
        .repo()
        .stats()
        .await
        .map_err(|e| clawrs_core::ClawrsError::internal(e.to_string()))?;
    Ok(Json(StatusResponse {
        version: VERSION,
        provider: state.runner.provider.id().0.clone(),
        default_model: state.default_model.clone(),
        stats,
    }))
}
