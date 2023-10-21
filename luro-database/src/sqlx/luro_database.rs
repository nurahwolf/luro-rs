use luro_model::configuration::Configuration;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct LuroDatabase {
    pub pool: Pool<Postgres>,
    pub twilight_client: Arc<twilight_http::Client>,
}

impl LuroDatabase {
    pub async fn new(config: &Configuration) -> Result<Self, sqlx::Error> {
        Ok(Self {
            pool: PgPoolOptions::new().max_connections(5).connect(&config.connection_string).await?,
            twilight_client: config.twilight_client.clone(),
        })
    }
}
