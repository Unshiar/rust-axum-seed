pub mod user;

use crate::database::state::AppState;
use crate::handlers::user::{create_user, delete_user, get_user};
use axum::Router;
use axum::routing::{delete, get, post};

pub fn register_handlers(state: AppState) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(state)
}
