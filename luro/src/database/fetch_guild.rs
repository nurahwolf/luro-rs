use twilight_model::id::{marker::GuildMarker, Id};

use crate::models::{interaction::InteractionResult, Guild};

use super::Database;

impl Database {
    pub async fn fetch_guild(&self, guild_id: Id<GuildMarker>) -> InteractionResult<Guild> {
        let guild = self.twilight_client.guild(guild_id).await?.model().await?;
        Ok(guild.into())
    }
}
