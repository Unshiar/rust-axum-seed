use axum::extract::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HealthStatus {
    #[schema(example = "ok")]
    status: String,
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health status is ok", body = HealthStatus),
    ),
    tag = "Health",
)]
pub async fn health_status() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: "ok".to_string(),
    })
}
