use luro_model::user::LuroUser;
use twilight_interactions::command::ResolvedUser;

use crate::CommandInteraction;

impl<T> CommandInteraction<T> {
    /// Get a specified user, else fall back to the interaction author

    pub async fn get_specified_user_or_author(&self, specified_user: Option<&ResolvedUser>) -> anyhow::Result<LuroUser> {
        match specified_user {
            Some(user_defined) => match self.database.get_user(user_defined.resolved.id.get() as i64).await? {
                Some(database_user) => Ok(database_user.into()),
                None => Ok(LuroUser::from(self.database.update_user(&self.twilight_client.user(user_defined.resolved.id).await?.model().await?).await?)),
            },
            None => self.get_interaction_author().await,
        }
    }
}
