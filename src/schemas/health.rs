use crate::handlers::health::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(health_status),
    components(schemas(
        HealthStatus
    )),
    tags((name = "Health", description = "Health Check Endpoints"))
)]
pub struct HealthApi;
