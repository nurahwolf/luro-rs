use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::{database::Error, user::User};

impl crate::database::Database {
    /// Will fetch a member if a guild ID is passed, otherwise will fetch user.
    pub async fn fetch_member_or_user(&self, guild_id: Option<Id<GuildMarker>>, user_id: Id<UserMarker>) -> Result<User, Error> {
        // Just fetch a user if no guild_id is present
        let Some(guild_id) = guild_id else {
            return self.fetch_user(user_id).await.map(|user| user.into());
        };

        // Attempt to fetch a member, if this fails raise a warning and return a user instead.
        match self.fetch_member(guild_id, user_id).await {
            Ok(member) => Ok(member.into()),
            Err(why) => {
                tracing::warn!(
                    ?why,
                    "Failed to fetch member `{user_id}` of guild `{guild_id}`, falling back to standard user"
                );
                self.fetch_user(user_id).await.map(|user| user.into())
            }
        }
    }
}
