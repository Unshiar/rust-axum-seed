pub mod health;
pub mod user;

use crate::database::state::AppState;
use crate::handlers::health::health_status;
use crate::handlers::user::{create_user, delete_user, get_user, get_users};
use crate::schemas::ApiDoc;
use axum::routing::{delete, get, post};
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use utoipa_swagger_ui::SwaggerUi;

fn configure_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
pub fn register_handlers(state: AppState) -> Router {
    let mut router = Router::new()
        // User routes
        .route("/user", post(create_user))
        .route("/user/{id}", get(get_user))
        .route("/user/{id}", delete(delete_user))
        .route("/users", get(get_users))
        // Health routes
        .route("/health", get(health_status))
        .with_state(state);

    // I assume Nginx proxying will be used in the production environment. The built-in Swagger is convenient for debugging.
    if cfg!(debug_assertions) {
        router = router
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
            .layer(configure_cors());
    }

    router
}
