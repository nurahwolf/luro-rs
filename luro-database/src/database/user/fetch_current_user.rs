use luro_model::types::User;
use twilight_model::id::{marker::GuildMarker, Id};

impl crate::Database {
    /// Fetch information on the bot's user
    pub async fn user_fetch_current_user(&self, guild_id: Option<Id<GuildMarker>>) -> anyhow::Result<User> {
        match guild_id {
            Some(guild_id) => {
                if let Ok(current_user) = self.member_fetch(self.current_user, guild_id).await {
                    return Ok(current_user);
                }
            }
            None => {
                if let Ok(current_user) = self.user_fetch(self.current_user).await {
                    return Ok(current_user);
                }
            }
        }

        tracing::warn!("Failed to get current user from database, falling back to querying the API");

        Ok(match guild_id {
            Some(guild_id) => (
                self.api_client.guild_member(guild_id, self.current_user).await?.model().await?,
                guild_id,
            )
                .into(),
            None => self.api_client.current_user().await?.model().await?.into(),
        })
    }
}
