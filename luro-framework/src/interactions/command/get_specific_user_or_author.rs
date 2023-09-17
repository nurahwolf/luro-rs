use luro_model::user::LuroUser;
use twilight_interactions::command::ResolvedUser;

use crate::CommandInteraction;

impl<T> CommandInteraction<T> {
    /// Get a specified user, else fall back to the interaction author
    /// Returns the user, their avatar and a nicely formatted name
    pub async fn get_specified_user_or_author(
        &self,
        specified_user: Option<&ResolvedUser>,
    ) -> anyhow::Result<LuroUser> {
        match specified_user {
            Some(user_defined) => self.database.get_user(&user_defined.resolved.id).await,
            None => self.get_interaction_author().await,
        }
    }
}
