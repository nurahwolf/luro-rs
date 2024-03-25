use luro_model::{database::Error, user::User};
use twilight_model::id::{marker::UserMarker, Id};

impl super::InteractionContext {
    /// Fetch a user, getting member data if the interaction is in a guild
    pub async fn fetch_user(&self, user_id: Id<UserMarker>) -> Result<User, Error> {
        self.gateway.database.fetch_member_or_user(self.interaction.guild_id, user_id).await
    }
}
