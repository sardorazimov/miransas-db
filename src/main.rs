use axum::{routing::get, Router};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL ayarlanmamış!");

    // Veritabanına bağlanıyoruz
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Miransas DB'ye bağlanamadı! Şifreyi kontrol et usta.");

    println!("✅ Miransas DB Bağlantısı Başarılı!");

    let app = Router::new()
        .route("/", get(|| async { "Miransas-Auth API v1.0 - Frankfurt Online" }))
        .with_state(pool);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 Sunucu {} üzerinde kükrüyor...", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
