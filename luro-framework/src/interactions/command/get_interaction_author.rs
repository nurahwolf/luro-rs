use luro_model::user::LuroUser;

use crate::{interactions::InteractionTrait, CommandInteraction};

impl<T> CommandInteraction<T> {
    /// Get and return useful information about the interaction author
    pub async fn get_interaction_author(&self) -> anyhow::Result<LuroUser> {
        self.database.get_user(&self.author_id()).await
    }
}
