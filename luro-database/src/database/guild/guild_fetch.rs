use luro_model::types::Guild;
use twilight_model::id::{Id, marker::GuildMarker};

impl crate::Database {
    pub async fn guild_fetch(&self, guild_id: Id<GuildMarker>) -> anyhow::Result<Guild> {
        if let Ok(Some(guild)) = self.driver.get_guild(guild_id).await {
            return Ok(guild)
        }

        tracing::warn!("Failed to find guild `{guild_id}` in the database, falling back to Twilight");
        let twilight_guild = self.api_client.guild(guild_id).await?.model().await?;

        if let Err(why) = self.driver.update_guild(twilight_guild.clone()).await {
            tracing::error!(why = ?why, "failed to sync guild `{guild_id}` to the database");
        }

        Ok(twilight_guild.into())
    }
}