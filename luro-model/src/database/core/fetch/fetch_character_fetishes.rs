use twilight_model::id::{marker::UserMarker, Id};

use crate::{character::CharacterFetish, database::Error};

impl crate::database::Database {
    pub async fn fetch_character_fetishes(&self, user_id: Id<UserMarker>, character_name: &str) -> Result<Vec<CharacterFetish>, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.fetch_character_fetishes(user_id, character_name).await {
            Ok(data) => Ok(data),
            Err(why) => {
                tracing::error!(?why, "Error fetching `{character_name}`'s fetishes");
                Err(Error::DriverFailure)
            }
        }

        #[cfg(not(feature = "database-sqlx"))]
        Err(Error::RequiresDriver)
    }
}
