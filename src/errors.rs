use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Serialize};
use serde_repr::*;

#[derive(Serialize)]
pub struct ApiError{
    #[serde(skip_serializing)]
    status: StatusCode,
    error: ApiErrorCode,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self)).into_response()
    }
}

#[derive(Serialize_repr)]
#[repr(u32)]
pub enum ApiErrorCode {
    InternalError = 3000,
    UserNotFound = 3001,
}

impl ApiError {
    pub fn user_not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            error: ApiErrorCode::UserNotFound,
            message: "User Not found".to_owned(),
        }
    }

    pub fn internal(message: String) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: ApiErrorCode::InternalError,
            message,
        }
    }
}
