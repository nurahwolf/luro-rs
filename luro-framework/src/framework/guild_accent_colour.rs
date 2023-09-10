use anyhow::anyhow;
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    /// fetches the accent colour of a guild if it is specified.
    /// Returns an error if we could not get any guild settings
    pub async fn guild_accent_colour(&self, guild_id: &Id<GuildMarker>) -> anyhow::Result<Option<u32>> {
        match self.database.get_guild(guild_id).await {
            Ok(mut guild_settings) => {
                // If a custom colour is present, return it
                if let Some(custom_accent_colour) = guild_settings.accent_colour_custom {
                    return Ok(Some(custom_accent_colour));
                };

                // If not, return the guild's colour. Returns None if the guild does not have an accent colour
                Ok(guild_settings.highest_role_colour().map(|x| x.0))
            }
            Err(why) => Err(anyhow!(
                "Unable to fetch guild settings for guild {guild_id} for the following reason: {why}"
            )),
        }
    }
}
