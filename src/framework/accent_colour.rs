use twilight_model::id::{marker::GuildMarker, Id};

use crate::ACCENT_COLOUR;

use super::LuroFramework;

impl LuroFramework {
    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    pub fn accent_colour(&self, guild_id: &Option<Id<GuildMarker>>) -> u32 {
        if let Some(guild_id) = guild_id {
            let guild_settings = self.data_guild.get(guild_id);

            if let Some(guild_settings) = guild_settings {
                // Check to see if a custom colour is defined
                if let Some(custom_accent_colour) = guild_settings.accent_colour_custom {
                    return custom_accent_colour;
                };

                if guild_settings.accent_colour != 0 {
                    return guild_settings.accent_colour;
                }
            }
        };

        ACCENT_COLOUR
    }
}
