use std::collections::hash_map::Entry;
use std::path::Path;
use tracing::warn;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::models::GuildSetting;
use crate::LuroContext;

use super::toml::LuroTOML;
use crate::GUILDSETTINGS_FILE_PATH;

impl LuroTOML for GuildSetting {}

impl GuildSetting {
    // This function makes sure guild settings are present on disk and in the cache
    pub async fn manage_guild_settings(
        ctx: &LuroContext,
        guild_id: Id<GuildMarker>,
        guild_settings: Option<Self>
    ) -> anyhow::Result<Self> {
        // new_guild_settings is set to true if we are specifying settings, so we know to flush it to disk.
        // TODO: Can I only call this if it is vacant from the cache?
        let data_from_disk = Self::get(Path::new(&format!(
            "{0}/{1}/guild_settings.toml",
            GUILDSETTINGS_FILE_PATH, guild_id
        )))
        .await?;
        let (mut guild_settings, new_settings) = match guild_settings {
            Some(guild_settings) => (guild_settings, true),
            None => (Self::default(), false)
        };

        if let Some(guild) = ctx.twilight_cache.guild(guild_id) {
            guild_settings.guild_name = guild.name().to_owned();
        }

        {
            let mut guild_data = ctx.guild_data.write();
            match guild_data.entry(guild_id) {
                Entry::Occupied(mut entry) => {
                    // If present and we have new settings passed to this function, replace the settings with what we defined
                    if new_settings {
                        let new_settings = entry.get_mut();
                        // Only overwrite if explicitly set
                        if let Some(accent_colour) = guild_settings.accent_colour_custom {
                            new_settings.accent_colour_custom = Some(accent_colour)
                        }

                        if let Some(moderator_actions_log_channel) = guild_settings.moderator_actions_log_channel {
                            new_settings.moderator_actions_log_channel = Some(moderator_actions_log_channel)
                        }

                        if let Some(discord_events_log_channel) = guild_settings.discord_events_log_channel {
                            new_settings.discord_events_log_channel = Some(discord_events_log_channel)
                        }

                        if let Some(accent_colour_custom) = guild_settings.accent_colour_custom {
                            new_settings.accent_colour_custom = Some(accent_colour_custom)
                        }
                        guild_settings = new_settings.clone()
                    }
                }
                Entry::Vacant(vacant) => {
                    guild_settings = data_from_disk;
                    // Only overwrite if explicitly set
                    if let Some(accent_colour) = guild_settings.accent_colour_custom {
                        guild_settings.accent_colour_custom = Some(accent_colour)
                    }

                    if let Some(moderator_actions_log_channel) = guild_settings.moderator_actions_log_channel {
                        guild_settings.moderator_actions_log_channel = Some(moderator_actions_log_channel)
                    }

                    if let Some(discord_events_log_channel) = guild_settings.discord_events_log_channel {
                        guild_settings.discord_events_log_channel = Some(discord_events_log_channel)
                    }

                    if let Some(accent_colour_custom) = guild_settings.accent_colour_custom {
                        guild_settings.accent_colour_custom = Some(accent_colour_custom)
                    }

                    vacant.insert(guild_settings.clone());
                }
            };
        }

        if new_settings {
            warn!("New settings are defined, flushing data to disk");
            Self::write(
                &guild_settings,
                Path::new(&format!("{0}/{1}/guild_settings.toml", GUILDSETTINGS_FILE_PATH, guild_id))
            )
            .await?;
        }

        Ok(guild_settings)
    }
}
