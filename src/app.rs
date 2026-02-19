use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::modules::{health, users};
use crate::state::AppState;

pub fn rust_saas(state: AppState) -> Router {
    Router::new()
        .nest("/api", api_routes())
        .merge(health::routes::health_routes())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .with_state(state)
}

fn api_routes() -> Router<AppState> {
    Router::new().nest("/users", users::routes::user_routes())
}
