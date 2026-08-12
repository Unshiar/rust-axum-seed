use crate::handlers::user::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_user),
    components(schemas(entities::user::Model, crate::errors::api::ApiError, crate::errors::codes::ApiErrorCodes)),
    tags((name = "Users", description = "User Management Endpoints"))
)]
pub struct UserApi;
