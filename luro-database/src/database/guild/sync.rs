use luro_model::types::Guild;

impl crate::Database {
    /// By default, data is returned from the database where possible. Because of this, it might be a little outdated.
    /// Calling Sync will use the passed twilight client to make a call to the Discord API, ensuring we have fresh data.
    /// The result is then written back to the database.
    ///
    /// You can also use this call to flush [GuildData] to the backend, if present.
    ///
    /// NOTE: This command does NOT fail, errors are raised to the console and data is simply not written to the database if unsuccessful.
    ///
    /// This also means if we can't update a user with new data from the API, such as no longer being in that guild, that no new data will be returned.
    pub async fn guild_sync<'a>(&'a self, guild: &'a mut Guild) -> &mut Guild {
        // Sync Luro specific data, if present
        if let Some(ref data) = guild.data {
            if let Err(why) = self.guild_update_data(data).await {
                tracing::warn!(why = ?why, "Failed to sync luro user data with the database")
            }
        }

        let twilight_guild = match self.api_client.guild(guild.guild_id).await {
            Ok(response) => match response.model().await {
                Ok(twilight_guild) => twilight_guild,
                Err(why) => {
                    tracing::error!(why = ?why, "Failed to convert received data from API into a Twilight guild");
                    return guild;
                }
            },
            Err(why) => {
                tracing::error!(why = ?why, "Failed to fetch guild using Twilight API");
                return guild;
            }
        };

        guild.afk_channel_id = twilight_guild.afk_channel_id;
        guild.afk_timeout = twilight_guild.afk_timeout;
        guild.application_id = twilight_guild.application_id;
        guild.approximate_member_count = twilight_guild.approximate_member_count;
        guild.approximate_presence_count = twilight_guild.approximate_presence_count;
        guild.banner = twilight_guild.banner;
        guild.channels = twilight_guild.channels.clone();
        guild.default_message_notifications = twilight_guild.default_message_notifications;
        guild.description = twilight_guild.description.clone();
        guild.discovery_splash = twilight_guild.discovery_splash;
        guild.emojis = twilight_guild.emojis.clone();
        guild.explicit_content_filter = twilight_guild.explicit_content_filter;
        guild.features = twilight_guild.features.clone();
        guild.icon = twilight_guild.icon;
        guild.joined_at = twilight_guild.joined_at;
        guild.large = twilight_guild.large;
        guild.max_members = twilight_guild.max_members;
        guild.max_presences = twilight_guild.max_presences;
        guild.max_video_channel_users = twilight_guild.max_video_channel_users;
        guild.member_count = twilight_guild.member_count;
        guild.members = twilight_guild.members.clone();
        guild.mfa_level = twilight_guild.mfa_level;
        guild.name = twilight_guild.name.clone();
        guild.nsfw_level = twilight_guild.nsfw_level;
        guild.owner = twilight_guild.owner;
        guild.owner_id = twilight_guild.owner_id;
        guild.owner_id = twilight_guild.owner_id;
        guild.permissions = twilight_guild.permissions;
        guild.preferred_locale = twilight_guild.preferred_locale.clone();
        guild.premium_progress_bar_enabled = twilight_guild.premium_progress_bar_enabled;
        guild.premium_subscription_count = twilight_guild.premium_subscription_count;
        guild.premium_tier = twilight_guild.premium_tier;
        guild.presences = twilight_guild.presences.clone();
        guild.public_updates_channel_id = twilight_guild.public_updates_channel_id;
        guild.roles = twilight_guild
            .roles
            .clone()
            .into_iter()
            .map(|role| (guild.guild_id, role).into())
            .collect();
        guild.rules_channel_id = twilight_guild.rules_channel_id;
        guild.safety_alerts_channel_id = twilight_guild.safety_alerts_channel_id;
        guild.splash = twilight_guild.splash;
        guild.stage_instances = twilight_guild.stage_instances.clone();
        guild.stickers = twilight_guild.stickers.clone();
        guild.system_channel_flags = twilight_guild.system_channel_flags;
        guild.system_channel_id = twilight_guild.system_channel_id;
        guild.threads = twilight_guild.threads.clone();
        guild.unavailable = twilight_guild.unavailable;
        guild.vanity_url_code = twilight_guild.vanity_url_code.clone();
        guild.verification_level = twilight_guild.verification_level;
        guild.voice_states = twilight_guild.voice_states.clone();
        guild.widget_channel_id = twilight_guild.widget_channel_id;
        guild.widget_enabled = twilight_guild.widget_enabled;

        if let Err(why) = self.driver.update_guild(&twilight_guild).await {
            tracing::warn!(why = ?why, "Failed to sync twilight user with the database")
        }

        guild
    }
}
