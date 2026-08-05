use crate::errors::codes::ApiErrorCodes;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct ApiError {
    #[serde(skip_serializing)]
    pub status: StatusCode,
    pub error: ApiErrorCodes,
    pub message: String, // user-friendly error message (for example for FE)
    pub details: Value,  // any additional error message
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self)).into_response()
    }
}
