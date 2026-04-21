use anyhow::{Context, Result};
use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::config::Config;

pub async fn connect(config: &Config) -> Result<PgPool> {
    // after_connect YOK - migration sırasında default search_path (public) gerekli
    PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await
        .context("failed to connect to PostgreSQL")
}

pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .context("failed to run SQLx migrations")?;

    // Migration bittikten sonra, role'ün default search_path'ini ayarla.
    // Artık yeni açılan her bağlantı _miransas'a öncelik verecek.
    // "public" de listede çünkü _sqlx_migrations orada kalmalı.
    sqlx::query("ALTER ROLE CURRENT_USER SET search_path TO _miransas, public")
        .execute(pool)
        .await
        .context("failed to set default search_path")?;

    Ok(())
}