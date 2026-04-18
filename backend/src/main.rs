#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "miransas_db=info,tower_http=info".into()),
        )
        .init();

    let config = miransas_db::config::Config::from_env()?;
    let pool = miransas_db::db::connect(&config).await?;
    miransas_db::db::run_migrations(&pool).await?;

    let addr = config.socket_addr()?;
    let state = miransas_db::state::AppState::new(config, pool);
    let app = miransas_db::build_router(state);

    tracing::info!(%addr, "starting miransas-db backend");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
