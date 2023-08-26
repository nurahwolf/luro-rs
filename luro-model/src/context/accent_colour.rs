use twilight_model::id::{Id, marker::GuildMarker};

use crate::{database::drivers::LuroDatabaseDriver, ACCENT_COLOUR};

use super::Context;

impl<D: LuroDatabaseDriver> Context<D> {
    /// Attempts to get the guild's accent colour, else falls back to getting the hardcoded accent colour
    pub async fn accent_colour(&self, guild_id: Option<&Id<GuildMarker>>) -> u32 {
        if let Some(guild_id) = guild_id {
            let guild_settings = self.database.get_guild(guild_id).await;

            if let Ok(guild_settings) = guild_settings {
                // Check to see if a custom colour is defined
                if let Some(custom_accent_colour) = guild_settings.accent_colour_custom {
                    return custom_accent_colour;
                };

                if let Some(colour) = guild_settings.accent_colour {
                    return colour;
                }
            }
        };

        ACCENT_COLOUR
    }
}

