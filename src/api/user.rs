use crate::errors::api::*;
use crate::errors::codes::*;
use crate::handlers::user::*;
use entities::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_user),
    components(schemas(user::Model, ApiError, ApiErrorCodes)),
    tags((name = "Users", description = "User Management Endpoints"))
)]
pub struct UserApi;
