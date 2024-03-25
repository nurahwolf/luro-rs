use twilight_model::id::{marker::UserMarker, Id};

use crate::{character::CharacterProfile, database::Error};

impl crate::database::Database {
    // Fetch a member from the database. Note that due to the need to query the database twice, this does not get roles automatically.
    pub async fn fetch_character(&self, user_id: Id<UserMarker>, name: &str) -> Result<Option<CharacterProfile>, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.fetch_character(user_id, name).await {
            Ok(data) => Ok(data),
            Err(why) => {
                tracing::error!(?why, "Error fetching character `{name}`");
                Err(Error::DriverFailure)
            }
        }

        #[cfg(not(feature = "database-sqlx"))]
        Err(Error::RequiresDriver)
    }
}
