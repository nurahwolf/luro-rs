use anyhow::Context;
use std::path::Path;
use tracing::warn;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::framework::LuroFramework;
use crate::models::GuildSetting;
use crate::LuroContext;

use crate::traits::toml::LuroTOML;
use crate::GUILDSETTINGS_FILE_PATH;

impl LuroTOML for GuildSetting {}

impl GuildSetting {
    /// This function just gets guild settings and ensures it is in Luro's context.
    pub async fn get_guild_settings(ctx: &LuroFramework, guild_id: &Id<GuildMarker>) -> anyhow::Result<Self> {
        // Check to see if our data is present. if it is, return early
        {
            if let Some(settings) = ctx.data_guild.get(guild_id) {
                return Ok(settings.clone());
            }
        }

        // If we got this far, we know we need to load from disk
        let guild_settings = Self::get(Path::new(&format!(
            "{0}/{1}/guild_settings.toml",
            GUILDSETTINGS_FILE_PATH, guild_id
        )))
        .await?;

        // Now insert it into our context
        {
            ctx.data_guild.insert(*guild_id, guild_settings.clone());
        }

        // Return the settings loaded from disk
        Ok(guild_settings)
    }

    // Get and modify some guild settings
    pub async fn modify_guild_settings(
        ctx: &LuroContext,
        guild_id: &Id<GuildMarker>,
        new_settings: &Self
    ) -> anyhow::Result<Self> {
        // This is only called to make sure they are present...
        let mut guild_settings = Self::get_guild_settings(ctx, guild_id).await?;

        {
            let mut new_guild_settings = ctx
                .data_guild
                .get_mut(guild_id)
                .context("Expected to have a guild in the cache!")?;

            if let Some(guild) = ctx.twilight_cache.guild(*guild_id) {
                guild_settings.guild_name = guild.name().to_owned();
            }

            new_guild_settings.accent_colour = new_settings.accent_colour;

            // Only overwrite if explicitly set
            if let Some(accent_colour_custom) = new_settings.accent_colour_custom {
                new_guild_settings.accent_colour_custom = Some(accent_colour_custom)
            }

            if let Some(catchall_log_channel) = new_settings.catchall_log_channel {
                new_guild_settings.catchall_log_channel = Some(catchall_log_channel)
            }

            if let Some(message_events_log_channel) = new_settings.message_events_log_channel {
                new_guild_settings.message_events_log_channel = Some(message_events_log_channel)
            }

            if let Some(moderator_actions_log_channel) = new_settings.moderator_actions_log_channel {
                new_guild_settings.moderator_actions_log_channel = Some(moderator_actions_log_channel)
            }

            if let Some(thread_events_log_channel) = new_settings.thread_events_log_channel {
                new_guild_settings.thread_events_log_channel = Some(thread_events_log_channel)
            }

            guild_settings = new_guild_settings.clone()
        }

        guild_settings.flush_to_disk(guild_id).await?;

        Ok(guild_settings)
    }

    pub async fn flush_to_disk(&self, guild_id: &Id<GuildMarker>) -> anyhow::Result<()> {
        warn!("New guild settings are defined, flushing data to disk");

        self.write(Path::new(&format!(
            "{0}/{1}/guild_settings.toml",
            GUILDSETTINGS_FILE_PATH, guild_id
        )))
        .await
    }
}
