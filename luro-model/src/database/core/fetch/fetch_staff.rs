use crate::{database::Error, user::UserContext};

impl crate::database::Database {
    pub async fn fetch_staff(&self) -> Result<Vec<UserContext>, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.fetch_staff().await {
            Ok(data) => match data.is_empty() {
                true => tracing::warn!("No staff were returned from the database, falling back to hardcoded."),
                false => return Ok(data),
            },
            Err(why) => tracing::error!(?why, "Error raised while trying to find staff"),
        };

        self.twilight_driver.fetch_staff().await
    }
}
