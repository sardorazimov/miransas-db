use axum::{
    extract::{Query, State},
    Json,
};

use crate::{
    errors::AppError,
    models::{AuditLog, AuditLogQuery},
    services,
    state::AppState,
};

/// GET /api/audit-logs?page=1&limit=50&resource_type=...&resource_id=...
///
/// All query parameters are optional. `limit` is clamped to 1–200.
/// Results are ordered by `created_at DESC`.
/// This handler does NOT write an audit log entry.
pub async fn list_audit_logs(
    State(state): State<AppState>,
    Query(query): Query<AuditLogQuery>,
) -> Result<Json<Vec<AuditLog>>, AppError> {
    Ok(Json(services::list_audit_logs(&state.pool, query).await?))
}
