use crate::errors::api::ApiError;
use crate::errors::codes::ApiErrorCodes;
use axum::http::StatusCode;
use entities::sea_orm::DbErr;

impl ApiError {
    pub fn internal_bd(er: &DbErr) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            error: ApiErrorCodes::InternalError,
            message: "Internal database error".to_owned(),
            details: serde_json::json!(er.to_string()),
        }
    }
}
