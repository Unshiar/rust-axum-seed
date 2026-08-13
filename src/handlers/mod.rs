pub mod health;
pub mod user;

use crate::api::ApiDoc;
use crate::database::state::AppState;
use crate::handlers::health::health_status;
use crate::handlers::user::{create_user, delete_user, get_user, get_users};
use axum::routing::{delete, get, post};
use axum::Router;
use utoipa_swagger_ui::SwaggerUi;

pub fn register_handlers(state: AppState) -> Router {
    Router::new()
        // User routes
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/user", post(create_user))
        .route("/user/{id}", get(get_user))
        .route("/user/{id}", delete(delete_user))
        .route("/users", get(get_users))
        // Health routes
        .route("/health", get(health_status))
        .with_state(state)
}
