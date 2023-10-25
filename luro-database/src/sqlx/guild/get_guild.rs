use tracing::{warn, error};
use twilight_model::{id::{marker::GuildMarker, Id}, guild::{AfkTimeout, DefaultMessageNotificationLevel, ExplicitContentFilter, MfaLevel, VerificationLevel, SystemChannelFlags, NSFWLevel}};

use crate::{DatabaseGuild, LuroDatabase, LuroGuild};

impl LuroDatabase {
    pub async fn get_all_guilds(&self) -> Result<Vec<DatabaseGuild>, sqlx::Error> {
        sqlx::query_as!(
            DatabaseGuild,
            "
            SELECT *
            FROM guilds
            "
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_guild(&self, guild_id: Id<GuildMarker>) -> anyhow::Result<LuroGuild> {
        let query = sqlx::query_file!("queries/guilds/get_guild.sql", guild_id.get() as i64)
            .fetch_optional(&self.pool)
            .await;

        // TODO: Finish this
        if let Ok(Some(guild)) = query {
            return Ok(LuroGuild {
                data: Default::default(),
                afk_channel_id: Default::default(),
                afk_timeout: AfkTimeout::from(guild.afk_timeout as u16),
                application_id: Default::default(),
                approximate_member_count: Default::default(),
                approximate_presence_count: Default::default(),
                banner: Default::default(),
                channels: Default::default(),
                default_message_notifications: DefaultMessageNotificationLevel::from(guild.default_message_notifications as u8),
                description: Default::default(),
                discovery_splash: Default::default(),
                emojis: Default::default(),
                explicit_content_filter: ExplicitContentFilter::from(guild.explicit_content_filter as u8),
                features: Default::default(),
                guild_id: guild.guild_id,
                icon: Default::default(),
                joined_at: Default::default(),
                large: Default::default(),
                max_members: Default::default(),
                max_presences: Default::default(),
                max_video_channel_users: Default::default(),
                member_count: Default::default(),
                members: Default::default(),
                mfa_level: MfaLevel::from(guild.mfa_level as u8),
                name: Default::default(),
                nsfw_level: NSFWLevel::from(guild.nsfw_level as u8),
                owner_id: Default::default(),
                owner: Default::default(),
                permissions: Default::default(),
                preferred_locale: Default::default(),
                premium_progress_bar_enabled: Default::default(),
                premium_subscription_count: Default::default(),
                premium_tier: Default::default(),
                presences: Default::default(),
                public_updates_channel_id: Default::default(),
                rules_channel_id: Default::default(),
                safety_alerts_channel_id: Default::default(),
                splash: Default::default(),
                stage_instances: Default::default(),
                stickers: Default::default(),
                system_channel_flags: SystemChannelFlags::from_bits_retain(guild.system_channel_flags as u64),
                system_channel_id: Default::default(),
                threads: Default::default(),
                unavailable: Default::default(),
                vanity_url_code: Default::default(),
                verification_level: VerificationLevel::from(guild.verification_level as u8),
                voice_states: Default::default(),
                widget_channel_id: Default::default(),
                widget_enabled: Default::default(),
            });
        }

        warn!("Failed to find guild `{guild_id}` in the database, falling back to Twilight");
        let twilight_guild = self.twilight_client.guild(guild_id).await?.model().await?;

        if let Err(why) = self.update_guild(twilight_guild.clone()).await {
            error!(why = ?why, "failed to sync guild `{guild_id}` to the database");
        }

        if let Ok(Some(guild)) = query {
            return Ok(LuroGuild {
                data: Default::default(),
                afk_channel_id: Default::default(),
                afk_timeout: AfkTimeout::from(guild.afk_timeout as u16),
                application_id: Default::default(),
                approximate_member_count: Default::default(),
                approximate_presence_count: Default::default(),
                banner: Default::default(),
                channels: Default::default(),
                default_message_notifications: DefaultMessageNotificationLevel::from(guild.default_message_notifications as u8),
                description: Default::default(),
                discovery_splash: Default::default(),
                emojis: Default::default(),
                explicit_content_filter: ExplicitContentFilter::from(guild.explicit_content_filter as u8),
                features: Default::default(),
                guild_id: guild.guild_id,
                icon: Default::default(),
                joined_at: Default::default(),
                large: Default::default(),
                max_members: Default::default(),
                max_presences: Default::default(),
                max_video_channel_users: Default::default(),
                member_count: Default::default(),
                members: Default::default(),
                mfa_level: MfaLevel::from(guild.mfa_level as u8),
                name: Default::default(),
                nsfw_level: NSFWLevel::from(guild.nsfw_level as u8),
                owner_id: Default::default(),
                owner: Default::default(),
                permissions: Default::default(),
                preferred_locale: Default::default(),
                premium_progress_bar_enabled: Default::default(),
                premium_subscription_count: Default::default(),
                premium_tier: Default::default(),
                presences: Default::default(),
                public_updates_channel_id: Default::default(),
                rules_channel_id: Default::default(),
                safety_alerts_channel_id: Default::default(),
                splash: Default::default(),
                stage_instances: Default::default(),
                stickers: Default::default(),
                system_channel_flags: SystemChannelFlags::from_bits_retain(guild.system_channel_flags as u64),
                system_channel_id: Default::default(),
                threads: Default::default(),
                unavailable: Default::default(),
                vanity_url_code: Default::default(),
                verification_level: VerificationLevel::from(guild.verification_level as u8),
                voice_states: Default::default(),
                widget_channel_id: Default::default(),
                widget_enabled: Default::default(),
            });
        }

        Ok(twilight_guild.into())
    }
}
