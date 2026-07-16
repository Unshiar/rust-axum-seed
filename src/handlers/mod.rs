pub mod health;
pub mod user;

use crate::database::state::AppState;
use crate::handlers::health::health_status;
use crate::handlers::user::{create_user, delete_user, get_user, get_users};
use axum::routing::{delete, get, post};
use axum::Router;

pub fn register_handlers(state: AppState) -> Router {
    Router::new()
        // User routes
        .route("/user", post(create_user))
        .route("/user/{id}", get(get_user))
        .route("/user/{id}", delete(delete_user))
        .route("/users", get(get_users))
        // Health routes
        .route("/health", get(health_status))
        .with_state(state)
}
