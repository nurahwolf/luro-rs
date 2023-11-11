use luro_model::types::User;
use twilight_model::id::{marker::UserMarker, Id};

use crate::{CommandInteraction, Luro};

impl CommandInteraction {
    /// Get a specified user, else fall back to the interaction author
    pub async fn get_specified_user_or_author(&self, specified_user: Option<Id<UserMarker>>) -> anyhow::Result<User> {
        match specified_user {
            Some(user) => self.fetch_user(user).await,
            None => self.fetch_user(self.author.user_id).await,
        }
    }
}
