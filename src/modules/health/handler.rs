use axum::{http::StatusCode, response::IntoResponse};

/// Root endpoint
pub async fn root() -> impl IntoResponse {
    (StatusCode::OK, "Rust SaaS Backend API")
}

/// Health check endpoint
/// Returns 200 OK if the service is healthy
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}
