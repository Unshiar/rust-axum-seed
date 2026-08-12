use crate::errors::api::ApiError;
use crate::errors::codes::InternalErrorCodes::DatabaseInternalError;
use crate::errors::codes::*;
use axum::http::StatusCode;

impl ApiError {
    pub fn internal_bd() -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: ApiErrorCodes::Internal(DatabaseInternalError),
            message: "Internal database error".to_owned(),
            details: serde_json::json!({}),
        }
    }
}
