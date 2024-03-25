use crate::{character::Character, database::Error};

impl crate::database::Database {
    // Fetch a member from the database. Note that due to the need to query the database twice, this does not get roles automatically.
    pub async fn update_character(&self, character: &Character<'_>) -> Result<(), Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.update_character(character).await {
            Ok(data) => Ok(data),
            Err(why) => {
                tracing::error!(?why, "Error updating `{character:#?}`");
                Err(Error::DriverFailure)
            }
        }

        #[cfg(not(feature = "database-sqlx"))]
        Err(Error::RequiresDriver)
    }
}
