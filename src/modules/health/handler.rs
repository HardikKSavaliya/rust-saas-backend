use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use sea_orm::ConnectionTrait;

use crate::error::{AppError, AppResult};
use crate::state::AppState;

pub async fn root() -> impl IntoResponse {
    (StatusCode::OK, "Rust SaaS Backend API")
}

pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

/// Database connectivity check - verifies the DB connection is alive
pub async fn db_health_check(State(state): State<AppState>) -> AppResult<impl IntoResponse> {
    state
        .db
        .execute_unprepared("SELECT 1")
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!({
        "status": "healthy",
        "database": "connected"
    })))
}

pub async fn example_error() -> AppError {
    AppError::ValidationError("Example validation error".to_string())
}

pub async fn example_success() -> impl IntoResponse {
    (StatusCode::OK, "Success")
}

pub async fn example_result() -> AppResult<impl IntoResponse> {
    let result: Result<String, String> = Err("Something went wrong".to_string());
    result
        .map_err(|e| AppError::internal(format!("Operation failed: {}", e)))
        .map(|data| (StatusCode::OK, data))
}
