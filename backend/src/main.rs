use axum::{routing::{get, post}, Router, middleware::from_fn};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;
use std::net::SocketAddr;

mod handlers;
mod models;
mod services;
mod middleware;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL missing!");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    let app = Router::new()
        .route("/api/query", post(handlers::query::handle_query))
        .route("/api/projects/create", post(handlers::project::create_project))
        .layer(from_fn(middleware::auth_layer::check_auth))
        .route("/health", get(|| async { "Miransas-DB is alive!" }))
        .layer(CorsLayer::permissive())
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("🚀 Miransas-DB Engine 3001 portunda ateşlendi!");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}