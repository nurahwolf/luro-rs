mod check_staff;
mod error;
mod fetch_channel;
mod fetch_guild;
mod fetch_interaction;
mod fetch_member;
mod fetch_message;
mod fetch_staff;
mod fetch_user;

pub use error::Error;

use crate::models::Config;

#[derive(thiserror::Error, Debug)]
pub enum DatabaseError {
    #[error(
        "No connection string was passed to the database, and it using a driver that requires one"
    )]
    NoConnectionString,
    #[cfg(feature = "database-sqlx")]
    #[error("SQLx had an error")]
    SqlxError(#[from] sqlx::Error),
}

#[derive(Debug, Clone)]
pub struct Database {
    #[cfg(feature = "database-sqlx")]
    pub pool: ::sqlx::Pool<::sqlx::Postgres>,
    pub twilight_client: std::sync::Arc<twilight_http::Client>,
}

impl Database {
    /// Create a new driver by fetching 'DATABASE_URL' from an environment variable
    pub async fn new(
        config: &Config,
        twilight_client: std::sync::Arc<twilight_http::Client>,
    ) -> Result<Self, DatabaseError> {
        #[cfg(feature = "database-sqlx")]
        let Some(ref connection_string) = config.connection_string
        else {
            return Err(DatabaseError::NoConnectionString);
        };

        Ok(Self {
            #[cfg(feature = "database-sqlx")]
            pool: ::sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(connection_string)
                .await?,
            twilight_client,
        })
    }
}
