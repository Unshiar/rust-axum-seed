use crate::errors::api::*;
use crate::errors::codes::*;
use crate::handlers::user::*;
use entities::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_user, create_user, delete_user),
    components(schemas(
        user::Model,
        UserId,
        CreateUserDto,
        ApiError,
        ApiErrorCodes)),
    tags((name = "Users", description = "User Management Endpoints"))
)]
pub struct UserApi;
