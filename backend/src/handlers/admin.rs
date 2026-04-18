use axum::{extract::State, Json};

use crate::{errors::AppError, models::AdminSummary, services, state::AppState};

pub async fn summary(State(state): State<AppState>) -> Result<Json<AdminSummary>, AppError> {
    Ok(Json(services::admin_summary(&state.pool).await?))
}
