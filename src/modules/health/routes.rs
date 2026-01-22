use axum::{Router, routing::get};

use super::handler;

/// Health check routes
pub fn health_routes() -> Router {
    Router::new()
        .route("/health", get(handler::health_check))
        .route("/", get(handler::root))
        // Example routes demonstrating error handling
        .route("/example/success", get(handler::example_success))
        .route("/example/error", get(handler::example_error))
        .route("/example/result", get(handler::example_result))
}
