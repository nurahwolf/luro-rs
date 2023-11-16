#![feature(let_chains)]

use twilight_model::id::{marker::UserMarker, Id};

mod database; // Functions for accessing data from the database
mod sync; // Functions for syncronising data from an event loop

/// Luro's Database. This struct takes driver modules to be able to generically store data on several types of backends.
/// Additionally, optional features for this crate can enable additional functionality, such as the twilight cache and twilight client.
/// Calls to new will always instance the database. Additional calls can be made to the building functions to setup for other features.
///
/// By default, this uses the Twilight client for updating the database with fresh data, and gracefully falling back to the API if the data does not exist.
/// If disabled, this will force the database to only query itself for data. Useful for if you can't reach the Discord API, however data will quickly grow stale.
#[derive(Debug)]
pub struct Database {
    /// User ID that represents the current user
    pub current_user: Id<UserMarker>,
    /// The API client used to query Discord for information. This is used as a fallback if no driver or cache is configured.
    ///
    /// Acceptable drivers:
    /// - twilight_http
    pub api_client: std::sync::Arc<twilight_http::Client>,
    /// The caching layer. This is always queried first if configured.
    ///
    /// Acceptable drivers:
    /// - twilight_inmemory_cache
    /// - none
    #[cfg(feature = "database-cache-twilight")]
    pub cache: twilight_cache_inmemory::InMemoryCache,
    /// The primary driver in which to fetch data. If not configured as a crate feature, this will use the Discord API using twilight.
    ///
    /// Acceptable drivers:
    /// - database_driver_sqlq
    /// - none
    #[cfg(feature = "database-driver-sqlx")]
    pub driver: luro_database_sqlx::SQLxDriver,
}

impl Database {
    pub async fn new(config: &luro_model::types::Configuration, current_user: Id<UserMarker>) -> anyhow::Result<Self> {
        Ok(Self {
            current_user,
            api_client: config.twilight_client.clone(),
            #[cfg(feature = "database-cache-twilight")]
            cache: twilight_cache_inmemory::InMemoryCache::new(),
            #[cfg(feature = "database-driver-sqlx")]
            driver: luro_database_sqlx::SQLxDriver::new().await?,
        })
    }
}
