use crate::error::{AppError, AppResult};
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

/// Example endpoint demonstrating error handling
/// This shows how to return errors from handlers
/// Since AppError implements IntoResponse, we can return it directly
pub async fn example_error() -> AppError {
    // Example: Return a validation error
    AppError::ValidationError("Example validation error".to_string())
}

/// Example endpoint demonstrating success response
pub async fn example_success() -> impl IntoResponse {
    (StatusCode::OK, "Success")
}

/// Example endpoint showing Result-based error handling
/// This demonstrates how to use AppResult<T> for handlers that return data
pub async fn example_result() -> AppResult<impl IntoResponse> {
    // Simulate some operation that might fail
    let result: Result<String, String> = Err("Something went wrong".to_string());

    // Convert to AppError using anyhow for error chaining
    result
        .map_err(|e| AppError::internal(format!("Operation failed: {}", e)))
        .map(|data| (StatusCode::OK, data))
}
