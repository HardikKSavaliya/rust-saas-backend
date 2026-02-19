use axum::{routing::get, Router};

use crate::state::AppState;

use super::handler;

pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list_users).post(handler::create_user))
        .route(
            "/{id}",
            get(handler::get_user)
                .put(handler::update_user)
                .delete(handler::delete_user),
        )
}
