use axum::{routing::get, Router};

use crate::state::AppState;

use super::handler;

pub fn health_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(handler::health_check))
        .route("/health/db", get(handler::db_health_check))
        .route("/", get(handler::root))
        .route("/example/success", get(handler::example_success))
        .route("/example/error", get(handler::example_error))
        .route("/example/result", get(handler::example_result))
}
