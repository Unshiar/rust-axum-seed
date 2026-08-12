use serde_repr::*;
use utoipa::ToSchema;

#[derive(Serialize_repr, ToSchema)]
#[repr(u32)]
pub enum ApiErrorCodes {
    InternalError = 3000,
    UserNotFound = 3001,
    InvalidCreateUserData = 3002,
}
