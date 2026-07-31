//! Axum HTTP gateway and application state wiring.

pub mod api;
pub mod error;
pub mod state;

use axum::Router;
use clawrs_config::AppConfig;
use state::AppState;
use std::path::Path;
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

pub async fn build_router(config: &AppConfig) -> anyhow::Result<Router> {
    let state = AppState::bootstrap(config).await?;
    Ok(api::routes(state).layer(TraceLayer::new_for_http()).layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    ))
}

pub fn attach_static(router: Router, static_dir: &Path) -> Router {
    let index = static_dir.join("index.html");
    if !index.exists() {
        return router;
    }
    router.fallback_service(
        ServeDir::new(static_dir)
            .not_found_service(ServeFile::new(index)),
    )
}

pub use state::AppState;
