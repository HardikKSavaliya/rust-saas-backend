use axum::{routing::get, Router};

use super::handler;

/// Health check routes
pub fn health_routes() -> Router {
    Router::new()
        .route("/health", get(handler::health_check))
        .route("/", get(handler::root))
}
