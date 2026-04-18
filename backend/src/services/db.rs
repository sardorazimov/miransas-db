use sqlx::{PgPool, Row, Column};
use crate::models::query::{QueryRequest, QueryResponse};
use std::time::Instant;

pub async fn execute_raw_sql(pool: &PgPool, req: QueryRequest) -> Result<QueryResponse, sqlx::Error> {
    let start = Instant::now();
    let schema_query = format!("SET search_path TO miransas_{}", req.project_id);
    sqlx::query(&schema_query).execute(pool).await?;

    let rows = sqlx::query(&req.sql).fetch_all(pool).await?;
    let mut columns = Vec::new();
    let mut result_rows = Vec::new();

    if !rows.is_empty() {
        columns = rows[0].columns().iter().map(|c| c.name().to_string()).collect();
        for row in rows {
            let mut row_map = serde_json::Map::new();
            for (i, col) in row.columns().iter().enumerate() {
                let val: serde_json::Value = row.try_get(i).unwrap_or(serde_json::Value::Null);
                row_map.insert(col.name().to_string(), val);
            }
            result_rows.push(serde_json::Value::Object(row_map));
        }
    }

    Ok(QueryResponse {
        columns,
        rows: result_rows,
        execution_time_ms: start.elapsed().as_millis(),
    })
}