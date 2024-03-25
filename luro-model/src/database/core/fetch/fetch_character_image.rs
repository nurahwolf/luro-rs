use twilight_model::id::{marker::UserMarker, Id};

use crate::{character::CharacterImage, database::Error};

impl crate::database::Database {
    // Fetch a member from the database. Note that due to the need to query the database twice, this does not get roles automatically.
    pub async fn fetch_character_image(
        &self,
        user_id: Id<UserMarker>,
        character_name: &str,
        id: i64,
    ) -> Result<Option<CharacterImage>, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.fetch_character_image(user_id, character_name, id).await {
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
