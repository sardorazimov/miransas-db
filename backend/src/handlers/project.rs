use axum::{extract::State, Json};
use sqlx::PgPool;
use crate::models::project::ProjectRequest;

pub async fn create_project(State(pool): State<PgPool>, Json(payload): Json<ProjectRequest>) -> Json<serde_json::Value> {
    let schema_name = format!("miransas_{}", payload.name.to_lowercase().replace(" ", "_"));
    let query = format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name);
    match sqlx::query(&query).execute(&pool).await {
        Ok(_) => Json(serde_json::json!({ "status": "success", "schema": schema_name })),
        Err(e) => Json(serde_json::json!({ "status": "error", "message": e.to_string() })),
    }
}