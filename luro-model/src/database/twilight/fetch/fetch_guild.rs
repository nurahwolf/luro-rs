use twilight_model::id::{marker::GuildMarker, Id};

use crate::{
    database::twilight::{Database, Error},
    guild::Guild,
};

impl Database {
    pub async fn fetch_guild(&self, guild_id: Id<GuildMarker>) -> Result<Guild, Error> {
        let twilight_guild = self.twilight_client.guild(guild_id).await?.model().await?;
        Ok(twilight_guild.into())
    }
}
