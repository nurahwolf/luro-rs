use luro_model::guild::LuroGuild;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::Framework;

impl Framework {
    /// fetches the accent colour of a guild if it is specified.
    /// Returns an error if we could not get any guild settings
    pub async fn guild_accent_colour(&self, guild_id: &Id<GuildMarker>) -> anyhow::Result<Option<u32>> {
        let mut guild = match self.database.get_guild(guild_id.get() as i64).await? {
            Some(guild) => LuroGuild::from(guild),
            None => LuroGuild::from(self.database.update_guild(self.twilight_client.guild(*guild_id).await?.model().await?).await?),
        };

        // If a custom colour is present, return it
        if let Some(custom_accent_colour) = guild.accent_colour_custom {
            return Ok(Some(custom_accent_colour));
        };

        // If not, return the guild's colour. Returns None if the guild does not have an accent colour
        Ok(guild.highest_role_colour().map(|x| x.0))
    }
}
