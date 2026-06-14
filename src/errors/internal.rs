use crate::errors::api::ApiError;
use crate::errors::codes::ApiErrorCodes;
use axum::http::StatusCode;

impl ApiError {
    pub fn internal_bd(message: String) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: ApiErrorCodes::InternalError,
            message,
        }
    }
}
