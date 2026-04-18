use axum::{middleware::from_fn_with_state, routing::get, Router};
use tower_http::trace::TraceLayer;

pub mod config;
pub mod db;
pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod services;
pub mod state;
pub mod utils;

use state::AppState;

pub fn build_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .route(
            "/projects",
            get(handlers::projects::list_projects).post(handlers::projects::create_project),
        )
        .route(
            "/databases",
            get(handlers::databases::list_databases).post(handlers::databases::create_database),
        )
        .route(
            "/secrets",
            get(handlers::secrets::list_secrets).post(handlers::secrets::create_secret),
        )
        .route("/admin/summary", get(handlers::admin::summary))
        .route_layer(from_fn_with_state(
            state.clone(),
            middleware::auth::require_admin,
        ));

    Router::new()
        .route("/health", get(handlers::health::health))
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
