use serde_repr::*;

#[derive(Serialize_repr)]
#[repr(u32)]
pub enum ApiErrorCodes {
    InternalError = 3000,
    UserNotFound = 3001,
    InvalidCreateUserData = 3002,
}
