use twilight_model::id::{marker::UserMarker, Id};

use crate::{character::CharacterFetish, database::Error};

impl crate::database::Database {
    pub async fn fetch_character_fetish(
        &self,
        user_id: Id<UserMarker>,
        character_name: &str,
        id: i64,
    ) -> Result<Option<CharacterFetish>, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.fetch_character_fetish(user_id, character_name, id).await {
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
