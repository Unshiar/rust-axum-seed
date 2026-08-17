use crate::errors::api::*;
use crate::errors::codes::*;
use axum::http::StatusCode;

impl ApiError {
    pub fn user_not_found() -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            error: ApiErrorCodes::UserNotFound,
            message: "User not found".to_owned(),
            details: serde_json::json!({}),
        }
    }

    pub fn invalid_create_user_data() -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            error: ApiErrorCodes::InvalidCreateUserData,
            message: "Invalid user data".to_owned(),
            details: serde_json::json!({}),
        }
    }
}
