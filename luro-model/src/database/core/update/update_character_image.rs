use crate::{character::CharacterImage, database::Error};

impl crate::database::Database {
    // Fetch a member from the database. Note that due to the need to query the database twice, this does not get roles automatically.
    pub async fn update_character_image(&self, img: &CharacterImage) -> Result<CharacterImage, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.update_character_image(img).await {
            Ok(data) => Ok(data),
            Err(why) => {
                tracing::error!(?why, "Error fetching `{img:#?}`'s characters");
                Err(Error::DriverFailure)
            }
        }

        #[cfg(not(feature = "database-sqlx"))]
        Err(Error::RequiresDriver)
    }
}
