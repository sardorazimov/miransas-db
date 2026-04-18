use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryRequest {
    pub sql: String,      
    pub project_id: String, 
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub columns: Vec<String>,
    pub rows: Vec<serde_json::Value>,
    pub execution_time_ms: u128,
}