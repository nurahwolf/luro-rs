use twilight_model::{
    application::interaction::Interaction,
    id::{marker::InteractionMarker, Id},
};

use crate::Database;

impl Database {
    pub async fn interaction_fetch(&self, interaction_id: Id<InteractionMarker>) -> anyhow::Result<Interaction> {
        self.driver.get_interaction(interaction_id).await
    }
}
