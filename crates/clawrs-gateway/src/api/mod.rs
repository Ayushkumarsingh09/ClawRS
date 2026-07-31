mod agents;
mod chat;
mod health;
mod sessions;

use crate::error::optional_api_key;
use crate::state::AppState;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn routes(state: AppState) -> Router {
    let api_key = state.api_key.clone();
    Router::new()
        .route("/health", get(health::health))
        .nest(
            "/api/v1",
            api_v1(state.clone()).layer(middleware::from_fn(move |req, next| {
                let key = api_key.clone();
                async move { optional_api_key(key, req, next).await }
            })),
        )
        .with_state(state)
}

fn api_v1(state: AppState) -> Router {
    Router::new()
        .route("/status", get(health::status))
        .route("/agents", get(agents::list).post(agents::create))
        .route("/agents/{id}", get(agents::get_one))
        .route("/sessions", get(sessions::list).post(sessions::create))
        .route("/sessions/{id}/messages", get(sessions::messages))
        .route("/sessions/{id}/chat", post(chat::chat))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn health_is_public() {
        let config = clawrs_config::AppConfig {
            listen: "127.0.0.1:0".into(),
            database_url: "sqlite::memory:".into(),
            static_dir: None,
            llm: clawrs_config::LlmConfig {
                provider: clawrs_config::LlmProviderKind::Mock,
                api_key: None,
                base_url: "http://localhost".into(),
                default_model: "mock".into(),
            },
            auth: clawrs_config::AuthConfig { api_key: None },
        };
        let state = AppState::bootstrap(&config).await.unwrap();
        let app = routes(state);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
