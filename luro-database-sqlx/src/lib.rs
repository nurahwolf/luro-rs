mod driver;
mod types;

/// Luro's SQLx driver, used for storing data in Postgres ONLY. There is no cache for this driver.
#[derive(Debug, Clone)]
pub struct SQLxDriver {
    /// Connection pool for interactiong with the Postgres database
    pub pool: ::sqlx::Pool<::sqlx::Postgres>,
}

impl SQLxDriver {
    /// Create a new driver by fetching 'DATABASE_URL' from an environment variable
    pub async fn new(connection_string: &str) -> anyhow::Result<Self> {
        Ok(Self {
            pool: sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(connection_string)
                .await?,
        })
    }
}
