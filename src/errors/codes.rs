use serde::Serialize;
use serde_repr::*;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(untagged)]
pub enum ApiErrorCodes {
    NotFound(NotFoundErrorCodes),
    Internal(InternalErrorCodes),
    Create(CreateErrorCodes),
}
#[derive(Serialize_repr, ToSchema)]
#[repr(u32)]
pub enum NotFoundErrorCodes {
    UserNotFound = 3001,
}
#[derive(Serialize_repr, ToSchema)]
#[repr(u32)]
pub enum CreateErrorCodes {
    InvalidCreateUserData = 4002,
}

#[derive(Serialize_repr, ToSchema)]
#[repr(u32)]
pub enum InternalErrorCodes {
    DatabaseInternalError = 5001,
}
