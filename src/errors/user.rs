use crate::errors::api::ApiError;
use crate::errors::codes::ApiErrorCodes;
use axum::http::StatusCode;

impl ApiError {
    pub fn user_not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            error: ApiErrorCodes::UserNotFound,
            message: "User Not found".to_owned(),
        }
    }
}
