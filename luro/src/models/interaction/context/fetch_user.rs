use twilight_model::id::{marker::UserMarker, Id};

use crate::models::{interaction::InteractionError, User};

impl super::InteractionContext {
    /// Fetch a user, getting member data if the interaction is in a guild
    pub async fn fetch_user(&self, user_id: Id<UserMarker>) -> Result<User, InteractionError> {
        match self.interaction.guild_id {
            Some(guild_id) => self.gateway.database.fetch_member(guild_id, user_id).await.map(|x|User::Member(x)),
            None => self.gateway.database.fetch_user(user_id).await,
        }
    }
}
