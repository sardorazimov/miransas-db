use axum::{extract::State, http::StatusCode, Json};

use crate::{
    errors::AppError,
    models::{CreateProjectRequest, Project},
    services,
    state::AppState,
};

pub async fn list_projects(State(state): State<AppState>) -> Result<Json<Vec<Project>>, AppError> {
    Ok(Json(services::list_projects(&state.pool).await?))
}

pub async fn create_project(
    State(state): State<AppState>,
    Json(input): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<Project>), AppError> {
    Ok((
        StatusCode::CREATED,
        Json(services::create_project(&state.pool, input).await?),
    ))
}
