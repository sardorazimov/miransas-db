use axum::{extract::State, http::StatusCode, Json};

use crate::{
    errors::AppError,
    models::{CreateDatabaseRequest, DatabaseMetadata},
    services,
    state::AppState,
};

pub async fn list_databases(
    State(state): State<AppState>,
) -> Result<Json<Vec<DatabaseMetadata>>, AppError> {
    Ok(Json(services::list_databases(&state.pool).await?))
}

pub async fn create_database(
    State(state): State<AppState>,
    Json(input): Json<CreateDatabaseRequest>,
) -> Result<(StatusCode, Json<DatabaseMetadata>), AppError> {
    Ok((
        StatusCode::CREATED,
        Json(services::create_database(&state.pool, input).await?),
    ))
}
