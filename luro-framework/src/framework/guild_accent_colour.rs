use twilight_model::id::{marker::GuildMarker, Id};

use crate::{Framework, Luro};

impl Framework {
    /// fetches the accent colour of a guild if it is specified.
    /// Returns an error if we could not get any guild settings
    pub async fn guild_accent_colour(&self, guild_id: &Id<GuildMarker>) -> anyhow::Result<Option<u32>> {
        let mut guild = self.get_guild(guild_id).await?;

        // If a custom colour is present, return it
        if let Some(custom_accent_colour) = guild.accent_colour_custom {
            return Ok(Some(custom_accent_colour));
        };

        // If not, return the guild's colour. Returns None if the guild does not have an accent colour
        Ok(guild.highest_role_colour().map(|x| x.0))
    }
}
