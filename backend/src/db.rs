use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::config::Config;

pub async fn connect(config: &Config) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .after_connect(|conn, _| {
            Box::pin(async move {
                sqlx::query("SET search_path TO _miransas, public")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&config.database_url)
        .await
        .context("failed to connect to PostgreSQL")
}

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    // _miransas şemasının var olduğundan emin ol, migration runner için.
    sqlx::query("CREATE SCHEMA IF NOT EXISTS _miransas")
        .execute(pool)
        .await
        .context("failed to ensure _miransas schema")?;

    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("failed to run SQLx migrations")
}