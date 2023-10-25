use futures_util::TryStreamExt;
use tracing::{warn, error};
use twilight_model::{id::{marker::GuildMarker, Id}, guild::{AfkTimeout, DefaultMessageNotificationLevel, ExplicitContentFilter, MfaLevel, VerificationLevel, SystemChannelFlags, NSFWLevel}, util::ImageHash};

use crate::{LuroDatabase, LuroGuild, LuroGuildData};

impl LuroDatabase {
    pub async fn get_all_guilds(&self) -> Result<Vec<LuroGuild>, sqlx::Error> {
        let mut guilds = vec![];
        let mut query = sqlx::query_file!("queries/guilds/get_guilds.sql")
            .fetch(&self.pool);

        while let Ok(Some(guild)) = query.try_next().await {
            guilds.push(LuroGuild {
                data: Some(LuroGuildData {
                    accent_colour: guild.accent_colour.map(|x|x as u32),
                    accent_colour_custom: guild.custom_accent_colour.map(|x|x as u32),
                }),
                afk_channel_id: Default::default(),
                afk_timeout: AfkTimeout::from(guild.afk_timeout as u16),
                application_id: Default::default(),
                approximate_member_count: guild.total_members.map(|x|x as u64),
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
                icon: guild.icon,
                joined_at: guild.joined_at,
                large: guild.large,
                max_members: guild.max_members.map(|x|x as u64),
                max_presences: Default::default(),
                max_video_channel_users: Default::default(),
                member_count: Default::default(),
                members: Default::default(),
                mfa_level: MfaLevel::from(guild.mfa_level as u8),
                name: Default::default(),
                nsfw_level: NSFWLevel::from(guild.nsfw_level as u8),
                owner_id: guild.owner_id,
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
            })
        }
        

        Ok(guilds)
    }

    pub async fn get_guild(&self, guild_id: Id<GuildMarker>) -> anyhow::Result<LuroGuild> {
        let query = sqlx::query_file!("queries/guilds/get_guild.sql", guild_id.get() as i64)
            .fetch_optional(&self.pool)
            .await;

        if let Ok(Some(guild)) = query {
            return Ok(LuroGuild {
                data: Some(LuroGuildData {
                    accent_colour: guild.accent_colour.map(|x|x as u32),
                    accent_colour_custom: guild.custom_accent_colour.map(|x|x as u32),
                }),
                afk_channel_id: Default::default(),
                afk_timeout: AfkTimeout::from(guild.afk_timeout as u16),
                application_id: Default::default(),
                approximate_member_count: guild.total_members.map(|x|x as u64),
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
                icon: guild.icon,
                joined_at: guild.joined_at,
                large: guild.large,
                max_members: guild.max_members.map(|x|x as u64),
                max_presences: Default::default(),
                max_video_channel_users: Default::default(),
                member_count: Default::default(),
                members: Default::default(),
                mfa_level: MfaLevel::from(guild.mfa_level as u8),
                name: Default::default(),
                nsfw_level: NSFWLevel::from(guild.nsfw_level as u8),
                owner_id: guild.owner_id,
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
                safety_alerts_channel_id: guild.safety_alerts_channel_id.map(|x| Id::new(x as u64)),
                splash: match guild.splash {
                    Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                    None => None
                },
                stage_instances: Default::default(),
                stickers: Default::default(),
                system_channel_flags: SystemChannelFlags::from_bits_retain(guild.system_channel_flags as u64),
                system_channel_id: guild.system_channel_id.map(|x| Id::new(x as u64)),
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
