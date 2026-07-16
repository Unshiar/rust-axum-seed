use axum::extract::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthStatus {
    status: String,
}

pub async fn health_status() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: "ok".to_string(),
    })
}
