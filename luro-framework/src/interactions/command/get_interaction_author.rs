use luro_database::LuroUser;

use crate::{CommandInteraction, InteractionTrait, Luro};

impl CommandInteraction {
    /// Get and return useful information about the interaction author
    pub async fn get_interaction_author(&self) -> anyhow::Result<LuroUser> {
        self.fetch_user(&self.author_id()).await
    }
}
