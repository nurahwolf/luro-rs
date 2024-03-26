impl crate::database::Database {
    /// Return how many channels are cached by the client. Returns zero if none are cached, or an error is raised by the driver.
    pub async fn count_channels(&self) -> i64 {
        let mut total_channels = 0;

        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.count_channels().await {
            Ok(channels) => total_channels += channels,
            Err(why) => tracing::error!(?why, "COUNT_CHANNELS: error checking the database"),
        }

        total_channels
    }
}
