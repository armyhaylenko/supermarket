pub mod crypto;

use color_eyre::Result;
use config::Config;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use sqlx::postgres::*;
use std::env;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
}

impl ConnectionConfig {
    #[instrument]
    pub fn from_env() -> Result<ConnectionConfig> {
        dotenv().ok();

        tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

        info!("Creating new config from environment...");

        let mut c = Config::new();
        c.merge(config::Environment::default())?;

        c.try_into().context("Creating new config from environment...")
    }

    pub async fn create_db_pool(&self) -> Result<PgPool> {
        info!("Creating database connection pool...");

        PgPool::connect_with(
            PgConnectOptions::new()
                .host("localhost")
                .port(5432)
                .username(&*env::var("POSTGRES_USER")?)
                .password(&*env::var("POSTGRES_PASSWORD")?)
                .database(&*env::var("POSTGRES_DATABASE")?),
        )
        .await
        .context("Creating database connection pool...")
    }
}
