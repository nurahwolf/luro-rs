use luro_database::LuroUser;
use twilight_interactions::command::ResolvedUser;

use crate::{CommandInteraction, Luro};

impl CommandInteraction {
    /// Get a specified user, else fall back to the interaction author

    pub async fn get_specified_user_or_author(&self, specified_user: Option<&ResolvedUser>) -> anyhow::Result<LuroUser> {
        match specified_user {
            Some(user) => self.fetch_user(&user.resolved.id).await,
            None => Ok(self.author.clone()),
        }
    }
}
