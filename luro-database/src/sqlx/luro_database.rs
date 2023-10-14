use luro_model::configuration::Configuration;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone, Debug)]
pub struct LuroDatabase {
    pub pool: Pool<Postgres>,
}

impl LuroDatabase {
    pub async fn new(config: &Configuration) -> Result<Self, sqlx::Error> {
        Ok(Self {
            pool: PgPoolOptions::new().max_connections(5).connect(&config.connection_string).await?,
        })
    }
}
