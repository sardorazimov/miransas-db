use sqlx::PgPool;

use crate::{
    errors::AppError,
    models::{AuditLog, AuditLogQuery},
};

/// List audit logs with optional filtering by resource_type and resource_id.
/// Results are ordered by created_at DESC. This function intentionally does NOT
/// write a new audit log entry to avoid an infinite loop.
pub async fn list_audit_logs(
    pool: &PgPool,
    query: AuditLogQuery,
) -> Result<Vec<AuditLog>, AppError> {
    let page = query.resolved_page();
    let limit = query.resolved_limit();
    let offset = (page - 1) * limit;

    let rows = sqlx::query_as::<_, AuditLog>(
        r#"
        SELECT id, action, resource_type, resource_id, message, created_at
        FROM   audit_logs
        WHERE  ($1::text IS NULL OR resource_type = $1)
          AND  ($2::uuid IS NULL OR resource_id   = $2)
        ORDER  BY created_at DESC
        LIMIT  $3 OFFSET $4
        "#,
    )
    .bind(query.resource_type)
    .bind(query.resource_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
