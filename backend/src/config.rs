use std::net::{IpAddr, SocketAddr};

use anyhow::{Context, Result};

#[derive(Clone)]
pub struct Config {
    pub app_host: String,
    pub app_port: u16,
    pub database_url: String,
    pub database_max_connections: u32,
    pub admin_token: String,
    pub secret_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let app_host = env_or("APP_HOST", "127.0.0.1");
        let app_port = env_or("APP_PORT", "3001")
            .parse::<u16>()
            .context("APP_PORT must be a valid port number")?;
        let database_url = required_env("DATABASE_URL")?;
        let database_max_connections = env_or("DATABASE_MAX_CONNECTIONS", "10")
            .parse::<u32>()
            .context("DATABASE_MAX_CONNECTIONS must be a positive integer")?;
        let admin_token = required_env("ADMIN_TOKEN")?;
        let secret_key = required_env("SECRET_KEY")?;

        if admin_token.len() < 16 {
            anyhow::bail!("ADMIN_TOKEN must be at least 16 characters");
        }

        if secret_key.len() < 32 {
            anyhow::bail!("SECRET_KEY must be at least 32 characters");
        }

        Ok(Self {
            app_host,
            app_port,
            database_url,
            database_max_connections,
            admin_token,
            secret_key,
        })
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        let ip = self
            .app_host
            .parse::<IpAddr>()
            .with_context(|| format!("APP_HOST must be an IP address, got {}", self.app_host))?;
        Ok(SocketAddr::new(ip, self.app_port))
    }
}

fn required_env(key: &str) -> Result<String> {
    std::env::var(key).with_context(|| format!("{key} is required"))
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}
