use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{ExportQuery, ProjectUserConfig, ResetPasswordRequest, TableDataResponse, UserSearchQuery},
    services,
    state::AppState,
};

/// GET /api/projects/:id/user-config
pub async fn get_config(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
) -> Result<Json<ProjectUserConfig>, AppError> {
    Ok(Json(services::users::get_config(&state.pool, project_id).await?))
}

/// PUT /api/projects/:id/user-config
pub async fn put_config(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Json(input): Json<crate::models::PutUserConfigRequest>,
) -> Result<Json<ProjectUserConfig>, AppError> {
    Ok(Json(
        services::users::put_config(&state.pool, project_id, input).await?,
    ))
}

/// GET /api/projects/:id/users?q=&page=&limit=
pub async fn list_users(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Query(q): Query<UserSearchQuery>,
) -> Result<Json<TableDataResponse>, AppError> {
    Ok(Json(services::users::list_users(&state.pool, project_id, q).await?))
}

/// GET /api/projects/:id/users/export?format=csv|json
pub async fn export_users(
    State(state): State<AppState>,
    Path(project_id): Path<Uuid>,
    Query(q): Query<ExportQuery>,
) -> Result<Response, AppError> {
    let (body, ct) = services::users::export_users(&state.pool, project_id, q).await?;
    Ok(([(header::CONTENT_TYPE, ct)], body).into_response())
}

/// GET /api/projects/:id/users/:user_id
pub async fn get_user(
    State(state): State<AppState>,
    Path((project_id, user_id)): Path<(Uuid, String)>,
) -> Result<Json<serde_json::Value>, AppError> {
    Ok(Json(
        services::users::get_user(&state.pool, project_id, &user_id).await?,
    ))
}

/// DELETE /api/projects/:id/users/:user_id
pub async fn delete_user(
    State(state): State<AppState>,
    Path((project_id, user_id)): Path<(Uuid, String)>,
) -> Result<StatusCode, AppError> {
    services::users::delete_user(&state.pool, project_id, &user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/projects/:id/users/:user_id/ban
pub async fn ban_user(
    State(state): State<AppState>,
    Path((project_id, user_id)): Path<(Uuid, String)>,
) -> Result<StatusCode, AppError> {
    services::users::ban_user(&state.pool, project_id, &user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/projects/:id/users/:user_id/unban
pub async fn unban_user(
    State(state): State<AppState>,
    Path((project_id, user_id)): Path<(Uuid, String)>,
) -> Result<StatusCode, AppError> {
    services::users::unban_user(&state.pool, project_id, &user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST /api/projects/:id/users/:user_id/password
pub async fn reset_password(
    State(state): State<AppState>,
    Path((project_id, user_id)): Path<(Uuid, String)>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<StatusCode, AppError> {
    services::users::reset_password(&state.pool, project_id, &user_id, req).await?;
    Ok(StatusCode::NO_CONTENT)
}
