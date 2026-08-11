pub use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags((name = "Users", description = "User Management Endpoints"))
)]
pub struct ApiDoc;
