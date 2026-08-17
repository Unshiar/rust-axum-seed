use serde_repr::Serialize_repr;
use utoipa::ToSchema;

#[derive(Serialize_repr, ToSchema)]
#[repr(u32)]
pub enum ApiErrorCodes {
    UserNotFound = 3001,
    InvalidCreateUserData = 4002,
    DatabaseInternalError = 5001,
}
