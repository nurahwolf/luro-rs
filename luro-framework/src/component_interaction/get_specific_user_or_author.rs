use luro_model::types::User;
use twilight_interactions::command::ResolvedUser;

use crate::{ComponentInteraction, Luro};

impl ComponentInteraction {
    /// Get a specified user, else fall back to the interaction author
    pub async fn get_specified_user_or_author(&self, specified_user: Option<&ResolvedUser>) -> anyhow::Result<User> {
        match specified_user {
            Some(user) => self.fetch_user(user.resolved.id).await,
            None => self.fetch_user(self.author.user_id).await,
        }
    }
}
