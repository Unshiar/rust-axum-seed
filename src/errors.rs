use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct ApiError{
    #[serde(skip_serializing)]
    status: StatusCode,
    #[serde(serialize_with = "serialize_enum_as_u32")]
    error: ApiErrorCode,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, Json(self)).into_response()
    }
}

#[derive(Serialize, Clone, Copy)]
pub enum ApiErrorCode {
    InternalError = 3000,
    UserNotFound = 3001,
}

fn serialize_enum_as_u32<S>(value: &ApiErrorCode, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u32(*value as u32)
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
