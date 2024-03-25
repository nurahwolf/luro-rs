use twilight_model::{
    application::interaction::Interaction,
    id::{marker::InteractionMarker, Id},
};

use crate::database::Error;

impl crate::database::Database {
    // Fetch a member from the database. Note that due to the need to query the database twice, this does not get roles automatically.
    pub async fn fetch_interaction(&self, interaction_id: Id<InteractionMarker>) -> Result<Interaction, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.fetch_interaction(interaction_id).await {
            Ok(data) => return Ok(data),
            Err(why) => tracing::error!(?why, "Error raised while attempting to fetch interaction `{interaction_id}`"),
        }

        Err(Error::RequiresDriver)
    }
}
