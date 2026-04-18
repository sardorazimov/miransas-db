use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        AdminSummary, CreateDatabaseRequest, CreateProjectRequest, CreateSecretRequest,
        DatabaseMetadata, Project, SecretMetadata,
    },
    utils::{crypto, time},
};

pub async fn list_projects(pool: &PgPool) -> Result<Vec<Project>, AppError> {
    let projects = sqlx::query_as::<_, Project>(
        r#"
        SELECT id, name, description, repository_url, created_at, updated_at
        FROM projects
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(projects)
}

pub async fn create_project(
    pool: &PgPool,
    input: CreateProjectRequest,
) -> Result<Project, AppError> {
    let name = required_text("name", input.name)?;

    let project = sqlx::query_as::<_, Project>(
        r#"
        INSERT INTO projects (name, description, repository_url)
        VALUES ($1, $2, $3)
        RETURNING id, name, description, repository_url, created_at, updated_at
        "#,
    )
    .bind(name)
    .bind(empty_to_none(input.description))
    .bind(empty_to_none(input.repository_url))
    .fetch_one(pool)
    .await?;

    insert_audit_log(
        pool,
        "create",
        "project",
        Some(project.id),
        Some(format!("created project {}", project.name)),
    )
    .await?;

    Ok(project)
}

pub async fn list_databases(pool: &PgPool) -> Result<Vec<DatabaseMetadata>, AppError> {
    let databases = sqlx::query_as::<_, DatabaseMetadata>(
        r#"
        SELECT id, project_id, name, engine, host, port, database_name, username, notes, created_at, updated_at
        FROM databases
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(databases)
}

pub async fn create_database(
    pool: &PgPool,
    input: CreateDatabaseRequest,
) -> Result<DatabaseMetadata, AppError> {
    let name = required_text("name", input.name)?;
    let engine = required_text("engine", input.engine)?;
    validate_port(input.port)?;

    let database = sqlx::query_as::<_, DatabaseMetadata>(
        r#"
        INSERT INTO databases (project_id, name, engine, host, port, database_name, username, notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, project_id, name, engine, host, port, database_name, username, notes, created_at, updated_at
        "#,
    )
    .bind(input.project_id)
    .bind(name)
    .bind(engine)
    .bind(empty_to_none(input.host))
    .bind(input.port)
    .bind(empty_to_none(input.database_name))
    .bind(empty_to_none(input.username))
    .bind(empty_to_none(input.notes))
    .fetch_one(pool)
    .await?;

    insert_audit_log(
        pool,
        "create",
        "database",
        Some(database.id),
        Some(format!("created database metadata {}", database.name)),
    )
    .await?;

    Ok(database)
}

pub async fn list_secrets(pool: &PgPool) -> Result<Vec<SecretMetadata>, AppError> {
    let secrets = sqlx::query_as::<_, SecretMetadata>(
        r#"
        SELECT id, project_id, name, notes, created_at, updated_at
        FROM secrets
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(secrets)
}

pub async fn create_secret(
    pool: &PgPool,
    secret_key: &str,
    input: CreateSecretRequest,
) -> Result<SecretMetadata, AppError> {
    let name = required_text("name", input.name)?;
    let value = required_text("value", input.value)?;
    let encrypted_value = crypto::encrypt(secret_key, &value)?;

    let secret = sqlx::query_as::<_, SecretMetadata>(
        r#"
        INSERT INTO secrets (project_id, name, value_encrypted, notes)
        VALUES ($1, $2, $3, $4)
        RETURNING id, project_id, name, notes, created_at, updated_at
        "#,
    )
    .bind(input.project_id)
    .bind(name)
    .bind(encrypted_value)
    .bind(empty_to_none(input.notes))
    .fetch_one(pool)
    .await?;

    insert_audit_log(
        pool,
        "create",
        "secret",
        Some(secret.id),
        Some(format!("created secret metadata {}", secret.name)),
    )
    .await?;

    Ok(secret)
}

pub async fn admin_summary(pool: &PgPool) -> Result<AdminSummary, AppError> {
    let summary = sqlx::query_as::<_, AdminSummary>(
        r#"
        SELECT
            (SELECT COUNT(*) FROM projects)::BIGINT AS project_count,
            (SELECT COUNT(*) FROM databases)::BIGINT AS database_count,
            (SELECT COUNT(*) FROM secrets)::BIGINT AS secret_count,
            (SELECT COUNT(*) FROM audit_logs)::BIGINT AS audit_log_count,
            NOW() AS generated_at
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(AdminSummary {
        generated_at: time::now(),
        ..summary
    })
}

async fn insert_audit_log(
    pool: &PgPool,
    action: &str,
    resource_type: &str,
    resource_id: Option<Uuid>,
    message: Option<String>,
) -> Result<(), AppError> {
    sqlx::query(
        r#"
        INSERT INTO audit_logs (action, resource_type, resource_id, message)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(action)
    .bind(resource_type)
    .bind(resource_id)
    .bind(message)
    .execute(pool)
    .await?;

    Ok(())
}

fn required_text(field: &str, value: String) -> Result<String, AppError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err(AppError::BadRequest(format!("{field} is required")));
    }

    Ok(trimmed.to_string())
}

fn empty_to_none(value: Option<String>) -> Option<String> {
    value.and_then(|text| {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_string())
        }
    })
}

fn validate_port(port: Option<i32>) -> Result<(), AppError> {
    if let Some(port) = port {
        if !(1..=65535).contains(&port) {
            return Err(AppError::BadRequest(
                "port must be between 1 and 65535".to_string(),
            ));
        }
    }

    Ok(())
}
