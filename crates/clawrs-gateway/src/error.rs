use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    Json,
};
use clawrs_core::ClawrsError;

pub struct ApiError(pub ClawrsError);

impl From<ClawrsError> for ApiError {
    fn from(value: ClawrsError) -> Self {
        Self(value)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self.0 {
            ClawrsError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            ClawrsError::NotFound { resource, id } => {
                (StatusCode::NOT_FOUND, format!("{resource} `{id}` not found"))
            }
            ClawrsError::PermissionDenied(msg) => (StatusCode::FORBIDDEN, msg.clone()),
            ClawrsError::RateLimited { retry_after_ms } => (
                StatusCode::TOO_MANY_REQUESTS,
                format!("retry after {retry_after_ms}ms"),
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

pub async fn optional_api_key(
    expected: Option<String>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, ApiError> {
    let Some(key) = expected else {
        return Ok(next.run(request).await);
    };
    let auth = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.strip_prefix("Bearer ").unwrap_or(s).to_string())
        .or_else(|| {
            request
                .headers()
                .get("x-clawrs-key")
                .and_then(|v| v.to_str().ok())
                .map(str::to_string)
        });
    if auth.as_deref() == Some(key.as_str()) {
        Ok(next.run(request).await)
    } else {
        Err(ClawrsError::PermissionDenied("invalid or missing API key".into()).into())
    }
}
