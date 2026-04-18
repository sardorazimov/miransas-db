use axum::{extract::State, http::StatusCode, Json};

use crate::{
    errors::AppError,
    models::{CreateSecretRequest, SecretMetadata},
    services,
    state::AppState,
};

pub async fn list_secrets(
    State(state): State<AppState>,
) -> Result<Json<Vec<SecretMetadata>>, AppError> {
    Ok(Json(services::list_secrets(&state.pool).await?))
}

pub async fn create_secret(
    State(state): State<AppState>,
    Json(input): Json<CreateSecretRequest>,
) -> Result<(StatusCode, Json<SecretMetadata>), AppError> {
    Ok((
        StatusCode::CREATED,
        Json(services::create_secret(&state.pool, &state.config.secret_key, input).await?),
    ))
}
