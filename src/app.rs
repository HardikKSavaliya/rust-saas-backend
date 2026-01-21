use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::modules::health;

/// Creates and configures the main application router
pub fn rust_saas() -> Router {
    Router::new()
        .nest("/api", api_routes())
        .merge(health::routes::health_routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
}

fn api_routes() -> Router {
    Router::new()
}
