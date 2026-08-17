use crate::errors::codes::ApiErrorCodes;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiError {
    #[serde(skip_serializing)]
    #[schema(ignore)]
    pub status: StatusCode,
    #[schema(value_type = u32)]
    pub error: ApiErrorCodes,
    pub message: String, // user-friendly error message (for example for FE)
    pub details: Value,  // any additional error message
}

impl ApiError {
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn add_details(self, details: Value) -> Self {
        Self { details, ..self }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self)).into_response()
    }
}
