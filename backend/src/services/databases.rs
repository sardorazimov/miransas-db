use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{CreateDatabaseRequest, DatabaseMetadata, UpdateDatabaseRequest},
    utils::crypto,
};

use super::shared::{empty_to_none, insert_audit_log, required_text, validate_port};

pub async fn list_databases(pool: &PgPool) -> Result<Vec<DatabaseMetadata>, AppError> {
    let rows = sqlx::query_as::<_, DatabaseMetadata>(
        r#"
        SELECT id, project_id, name, engine, host, port,
               database_name, username, notes, created_at, updated_at
        FROM   databases
        ORDER  BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn create_database(
    pool: &PgPool,
    secret_key: &str,
    input: CreateDatabaseRequest,
) -> Result<DatabaseMetadata, AppError> {
    let name = required_text("name", input.name)?;
    let engine = required_text("engine", input.engine)?;
    validate_port(input.port)?;

    // Encrypt the connection URL if provided.
    let connection_url_encrypted = input
        .connection_url
        .filter(|u| !u.trim().is_empty())
        .map(|u| crypto::encrypt(secret_key, &u))
        .transpose()?;

    let db = sqlx::query_as::<_, DatabaseMetadata>(
        r#"
        INSERT INTO databases
            (project_id, name, engine, host, port,
             database_name, username, notes, connection_url_encrypted)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id, project_id, name, engine, host, port,
                  database_name, username, notes, created_at, updated_at
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
    .bind(connection_url_encrypted)
    .fetch_one(pool)
    .await?;

    insert_audit_log(
        pool,
        "create",
        "database",
        Some(db.id),
        Some(format!("registered database {}", db.name)),
    )
    .await?;

    Ok(db)
}

/// Update a database entry. Absent fields keep their existing value;
/// an empty string sets the field to NULL (except `name` and `engine`).
pub async fn update_database(
    pool: &PgPool,
    secret_key: &str,
    id: Uuid,
    input: UpdateDatabaseRequest,
) -> Result<DatabaseMetadata, AppError> {
    #[derive(sqlx::FromRow)]
    struct DatabaseFull {
        name: String,
        engine: String,
        project_id: Option<Uuid>,
        host: Option<String>,
        port: Option<i32>,
        database_name: Option<String>,
        username: Option<String>,
        notes: Option<String>,
        connection_url_encrypted: Option<String>,
    }

    let existing = sqlx::query_as::<_, DatabaseFull>(
        r#"
        SELECT name, engine, project_id, host, port,
               database_name, username, notes, connection_url_encrypted
        FROM   databases
        WHERE  id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("database {id} not found")))?;

    let name = match input.name {
        Some(n) => required_text("name", n)?,
        None => existing.name,
    };
    let engine = match input.engine {
        Some(e) => required_text("engine", e)?,
        None => existing.engine,
    };
    let project_id = if input.project_id.is_some() {
        input.project_id
    } else {
        existing.project_id
    };
    let host = if input.host.is_some() {
        empty_to_none(input.host)
    } else {
        existing.host
    };
    let port = if input.port.is_some() {
        validate_port(input.port)?;
        input.port
    } else {
        existing.port
    };
    let database_name = if input.database_name.is_some() {
        empty_to_none(input.database_name)
    } else {
        existing.database_name
    };
    let username = if input.username.is_some() {
        empty_to_none(input.username)
    } else {
        existing.username
    };
    let notes = if input.notes.is_some() {
        empty_to_none(input.notes)
    } else {
        existing.notes
    };
    let connection_url_encrypted = match input.connection_url {
        None => existing.connection_url_encrypted,
        Some(ref u) if u.trim().is_empty() => None,
        Some(u) => Some(crypto::encrypt(secret_key, &u)?),
    };

    let db = sqlx::query_as::<_, DatabaseMetadata>(
        r#"
        UPDATE databases
        SET    name = $2,
               engine = $3,
               project_id = $4,
               host = $5,
               port = $6,
               database_name = $7,
               username = $8,
               notes = $9,
               connection_url_encrypted = $10
        WHERE  id = $1
        RETURNING id, project_id, name, engine, host, port,
                  database_name, username, notes, created_at, updated_at
        "#,
    )
    .bind(id)
    .bind(&name)
    .bind(&engine)
    .bind(project_id)
    .bind(host)
    .bind(port)
    .bind(database_name)
    .bind(username)
    .bind(notes)
    .bind(connection_url_encrypted)
    .fetch_one(pool)
    .await?;

    insert_audit_log(
        pool,
        "update",
        "database",
        Some(id),
        Some(format!("updated database {name}")),
    )
    .await?;

    Ok(db)
}

/// Delete a database entry by id. Returns 404 if not found.
pub async fn delete_database(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM databases WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("database {id} not found")));
    }

    insert_audit_log(
        pool,
        "delete",
        "database",
        Some(id),
        Some(format!("deleted database {id}")),
    )
    .await?;

    Ok(())
}
