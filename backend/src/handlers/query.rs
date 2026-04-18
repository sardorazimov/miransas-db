use axum::{extract::State, Json, response::IntoResponse};
use sqlx::PgPool;
use crate::models::query::QueryRequest;
use crate::services::db;

pub async fn handle_query(State(pool): State<PgPool>, Json(payload): Json<QueryRequest>) -> impl IntoResponse {
    match db::execute_raw_sql(&pool, payload).await {
        Ok(res) => (axum::http::StatusCode::OK, Json(res)).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response(),
    }
}