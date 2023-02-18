use tracing::warn;
use twilight_model::id::{Id, marker::GuildMarker};

use crate::{ACCENT_COLOUR, Luro};

impl Luro {
    pub async fn accent_colour(&self, guild: Option<Id<GuildMarker>>) -> u32 {
        match guild {
            Some(guild) => {
                let all_guild_settings = self.guild_settings.read().await;
            
                let guild_settings = match all_guild_settings.guilds.get(&guild.to_string()) {
                    Some(ok) => ok,
                    None => {
                        warn!("No guild settings defined for guild");
                        return ACCENT_COLOUR
                    },
                };
            
                if let Some(custom_accent_colour_defined) = guild_settings.accent_colour_custom {
                    custom_accent_colour_defined
                } else {
                    guild_settings.accent_colour
                }
            },
            None => ACCENT_COLOUR,
        }
    }
}