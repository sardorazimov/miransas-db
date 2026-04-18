use axum::Json;

use crate::{models::HealthResponse, utils::time};

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "miransas-db",
        timestamp: time::now(),
    })
}
