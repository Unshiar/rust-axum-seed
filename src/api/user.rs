use crate::errors::api::*;
use crate::errors::codes::*;
use crate::handlers::user::*;
use entities::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_user, get_users, create_user, delete_user),
    components(schemas(
        user::UserResponseDto,
        UserId,
        CreateUserDto,
        ApiError,
        ApiErrorCodes)),
    tags((name = "Users", description = "User Management Endpoints"))
)]
pub struct UserApi;
