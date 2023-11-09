mod driver;
mod types;

/// Luro's SQLx driver, used for storing data in Postgres ONLY. There is no cache for this driver.
#[derive(Debug, Clone)]
pub struct SQLxDriver {
    /// Connection pool for interactiong with the Postgres database
    pub pool: ::sqlx::Pool<::sqlx::Postgres>,
}

impl SQLxDriver {
    /// Create a new driver by passing in a configuration context
    pub async fn new_config(config: &luro_model::configuration::Configuration) -> Result<Self, sqlx::Error> {
        Ok(Self {
            pool: sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&config.connection_string)
                .await?,
        })
    }

    /// Create a new driver by fetching 'DATABASE_URL' from an environment variable
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            pool: sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(&std::env::var("DATABASE_URL")?)
                .await?,
        })
    }
}
