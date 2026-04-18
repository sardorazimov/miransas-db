use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use miransas_db::{build_router, config::Config, state::AppState};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

fn test_state() -> AppState {
    let config = Config {
        app_host: "127.0.0.1".to_string(),
        app_port: 3001,
        database_url: "postgres://postgres:postgres@localhost:5432/miransas_db_test".to_string(),
        database_max_connections: 1,
        admin_token: "test-admin-token-123".to_string(),
        secret_key: "test-secret-key-with-at-least-32-chars".to_string(),
    };
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect_lazy(&config.database_url)
        .expect("test database URL should be valid");

    AppState::new(config, pool)
}

#[tokio::test]
async fn health_endpoint_works() {
    let app = build_router(test_state());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn protected_route_rejects_missing_bearer_token() {
    let app = build_router(test_state());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/projects")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
