use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        AddCheckConstraintRequest, AddColumnRequest, AddForeignKeyRequest, AlterColumnTypeRequest,
        CreateIndexRequest, CreateTableRequest, DropColumnQuery, DropTableQuery,
        RenameColumnRequest, RenameTableRequest, TableStructureResponse,
    },
    services,
    state::AppState,
};

/// GET /api/projects/:project_id/schema/tables/:table
pub async fn get_table_structure(
    State(state): State<AppState>,
    Path((project_id, table)): Path<(Uuid, String)>,
) -> Result<Json<TableStructureResponse>, AppError> {
    Ok(Json(
        services::schema::get_table_structure(&state.pool, project_id, &table).await?,
    ))
}

/// POST /api/projects/:project_id/schema/tables
pub async fn create_table(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Json(input): Json<CreateTableRequest>,
) -> Result<StatusCode, AppError> {
    services::schema::create_table(&state.pool, project_id, input).await?;
    Ok(StatusCode::CREATED)
}

/// DELETE /api/projects/:project_id/schema/tables/:table
pub async fn drop_table(
    State(state): State<AppState>,
    Path((project_id, table)): Path<(Uuid, String)>,
    Query(params): Query<DropTableQuery>,
) -> Result<StatusCode, AppError> {
    services::schema::drop_table(&state.pool, project_id, &table, params.cascade.unwrap_or(false))
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/projects/:project_id/schema/tables/:table/rename
pub async fn rename_table(
    State(state): State<AppState>,
    Path((project_id, table)): Path<(Uuid, String)>,
    Json(input): Json<RenameTableRequest>,
) -> Result<StatusCode, AppError> {
    services::schema::rename_table(&state.pool, project_id, &table, input).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/projects/:project_id/schema/tables/:table/columns
pub async fn add_column(
    State(state): State<AppState>,
    Path((project_id, table)): Path<(Uuid, String)>,
    Json(input): Json<AddColumnRequest>,
) -> Result<StatusCode, AppError> {
    services::schema::add_column(&state.pool, project_id, &table, input).await?;
    Ok(StatusCode::CREATED)
}

/// DELETE /api/projects/:project_id/schema/tables/:table/columns/:column
pub async fn drop_column(
    State(state): State<AppState>,
    Path((project_id, table, column)): Path<(Uuid, String, String)>,
    Query(params): Query<DropColumnQuery>,
) -> Result<StatusCode, AppError> {
    services::schema::drop_column(
        &state.pool,
        project_id,
        &table,
        &column,
        params.cascade.unwrap_or(false),
    )
    .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/projects/:project_id/schema/tables/:table/columns/:column/rename
pub async fn rename_column(
    State(state): State<AppState>,
    Path((project_id, table, column)): Path<(Uuid, String, String)>,
    Json(input): Json<RenameColumnRequest>,
) -> Result<StatusCode, AppError> {
    services::schema::rename_column(&state.pool, project_id, &table, &column, input).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/projects/:project_id/schema/tables/:table/columns/:column/type
pub async fn alter_column_type(
    State(state): State<AppState>,
    Path((project_id, table, column)): Path<(Uuid, String, String)>,
    Json(input): Json<AlterColumnTypeRequest>,
) -> Result<StatusCode, AppError> {
    services::schema::alter_column_type(&state.pool, project_id, &table, &column, input).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/projects/:project_id/schema/tables/:table/foreign-keys
pub async fn add_foreign_key(
    State(state): State<AppState>,
    Path((project_id, table)): Path<(Uuid, String)>,
    Json(input): Json<AddForeignKeyRequest>,
) -> Result<StatusCode, AppError> {
    services::schema::add_foreign_key(&state.pool, project_id, &table, input).await?;
    Ok(StatusCode::CREATED)
}

/// DELETE /api/projects/:project_id/schema/tables/:table/constraints/:constraint_name
pub async fn drop_constraint(
    State(state): State<AppState>,
    Path((project_id, table, constraint_name)): Path<(Uuid, String, String)>,
) -> Result<StatusCode, AppError> {
    services::schema::drop_constraint(&state.pool, project_id, &table, &constraint_name).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/projects/:project_id/schema/tables/:table/check-constraints
pub async fn add_check_constraint(
    State(state): State<AppState>,
    Path((project_id, table)): Path<(Uuid, String)>,
    Json(input): Json<AddCheckConstraintRequest>,
) -> Result<StatusCode, AppError> {
    services::schema::add_check_constraint(&state.pool, project_id, &table, input).await?;
    Ok(StatusCode::CREATED)
}

/// POST /api/projects/:project_id/schema/tables/:table/indexes
pub async fn create_index(
    State(state): State<AppState>,
    Path((project_id, table)): Path<(Uuid, String)>,
    Json(input): Json<CreateIndexRequest>,
) -> Result<StatusCode, AppError> {
    services::schema::create_index(&state.pool, project_id, &table, input).await?;
    Ok(StatusCode::CREATED)
}

/// DELETE /api/projects/:project_id/schema/indexes/:index_name
pub async fn drop_index(
    State(state): State<AppState>,
    Path((project_id, index_name)): Path<(Uuid, String)>,
) -> Result<StatusCode, AppError> {
    services::schema::drop_index(&state.pool, project_id, &index_name).await?;
    Ok(StatusCode::NO_CONTENT)
}
