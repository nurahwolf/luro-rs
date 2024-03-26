use crate::{character::CharacterFetish, database::Error};

impl crate::database::Database {
    // Fetch a member from the database. Note that due to the need to query the database twice, this does not get roles automatically.
    pub async fn update_character_fetish(&self, fetish: &CharacterFetish) -> Result<CharacterFetish, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.update_character_fetish(fetish).await {
            Ok(data) => Ok(data),
            Err(why) => {
                tracing::error!(?why, "Error fetching `{fetish:#?}`'s characters");
                Err(Error::DriverFailure)
            }
        }

        #[cfg(not(feature = "database-sqlx"))]
        Err(Error::RequiresDriver)
    }
}
