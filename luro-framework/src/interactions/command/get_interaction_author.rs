use luro_model::user::LuroUser;

use crate::{interactions::InteractionTrait, CommandInteraction};

impl<T> CommandInteraction<T> {
    /// Get and return useful information about the interaction author
    pub async fn get_interaction_author(&self) -> anyhow::Result<LuroUser> {
        match self.database.get_user(self.author_id().get() as i64).await? {
            Some(database_user) => Ok(database_user.into()),
            None => Ok(LuroUser::from(self.database.update_user(&self.twilight_client.user(self.author_id()).await?.model().await?).await?)),
        }
    }
}
