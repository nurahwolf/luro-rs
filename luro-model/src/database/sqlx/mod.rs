use twilight_model::util::{datetime::TimestampParseError, image_hash::ImageHashParseError};

use crate::config::Config;

mod fetch;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The SQLx driver MUST have a connection string passed, so that it knows what database to connect to.")]
    NoConnectionString,
    #[error("The SQLx driver itself had an error")]
    SqlxError(#[from] ::sqlx::Error),
    #[error("A image returned from the database failed to be converted to a Twilight type")]
    ImageHashParseError(#[from] ImageHashParseError),
    #[error("A timestamp returned from the database failed to be converted to a Twilight type")]
    TimestampParseError(#[from] TimestampParseError),
}

#[derive(Debug)]
pub struct Database {
    pub pool: ::sqlx::Pool<::sqlx::Postgres>,
}

impl Database {
    /// Create a new database instance
    pub async fn new(config: &Config) -> Result<Self, Error> {
        let Some(ref connection_string) = config.connection_string else {
            return Err(Error::NoConnectionString);
        };

        Ok(Self {
            pool: ::sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect(connection_string)
                .await?,
        })
    }
}
