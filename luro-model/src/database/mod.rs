use std::sync::Arc;

use twilight_http::Client;

use crate::config::Config;

/// Core module, used to passing down requests to the drivers.
mod core;
#[cfg(feature = "database-sqlx")]
/// A module for fetching data using the SQLx driver.
pub mod sqlx;
/// A module for fetching data using the twilight HTTP client.
pub mod twilight;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The database driver had a major failure")]
    DriverFailure,
    #[error("A database driver is needed in order to support this data type")]
    RequiresDriver,
    #[error("Twilight failed to deserialize a response")]
    DeserializeBodyError(#[from] twilight_http::response::DeserializeBodyError),
    #[error("The API client had an error while communicating with the Discord API")]
    TwilightClient(#[from] twilight_http::Error),
}

#[derive(Debug)]
pub struct Database {
    #[cfg(feature = "database-sqlx")]
    pub sqlx_driver: crate::database::sqlx::Database,
    pub twilight_driver: crate::database::twilight::Database,
    pub twilight_client: Arc<Client>,
}

impl Database {
    pub async fn new(#[cfg(feature = "database-sqlx")] config: &Config, twilight_client: Arc<Client>) -> Result<Self, Error> {
        Ok(Self {
            #[cfg(feature = "database-sqlx")]
            sqlx_driver: match crate::database::sqlx::Database::new(config).await {
                Ok(data) => data,
                Err(why) => {
                    tracing::error!(?why, "Failed to start the database driver.");
                    return Err(Error::DriverFailure);
                }
            },
            twilight_driver: crate::database::twilight::Database::new(twilight_client.clone()),
            twilight_client,
        })
    }
}
