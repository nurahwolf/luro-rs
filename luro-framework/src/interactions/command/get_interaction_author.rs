use luro_model::user::LuroUser;

use crate::{CommandInteraction, Luro, InteractionTrait};

impl CommandInteraction {
    /// Get and return useful information about the interaction author
    pub async fn get_interaction_author(&self) -> anyhow::Result<LuroUser> {
        self.get_user(&self.author_id()).await
    }
}
