use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::{message::Sticker, Channel, StageInstance},
    gateway::presence::Presence,
    guild::{
        AfkTimeout, DefaultMessageNotificationLevel, Emoji, ExplicitContentFilter, Guild, GuildFeature, MfaLevel, NSFWLevel, Permissions,
        PremiumTier, SystemChannelFlags, VerificationLevel,
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, UserMarker},
        Id,
    },
    util::{ImageHash, Timestamp},
    voice::VoiceState,
};

use crate::{DatabaseGuild, LuroGuildData};

mod alert_channels;
mod fetch_role;
mod get_everyone_role;
mod get_member_highest_role;
mod get_members;
mod guild_id;
mod is_owner;
mod new;
mod owner_id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuroGuild {
    pub data: Option<LuroGuildData>,
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: AfkTimeout,
    pub application_id: Option<Id<ApplicationMarker>>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub banner: Option<ImageHash>,
    pub channels: Vec<Channel>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<ImageHash>,
    pub emojis: Vec<Emoji>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<GuildFeature>,
    pub guild_id: i64,
    pub icon: Option<ImageHash>,
    pub joined_at: Option<Timestamp>,
    pub large: bool,
    pub max_members: Option<u64>,
    pub max_presences: Option<u64>,
    pub max_video_channel_users: Option<u64>,
    pub member_count: Option<u64>,
    pub members: Vec<Id<UserMarker>>,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub nsfw_level: NSFWLevel,
    pub owner_id: i64,
    pub owner: Option<bool>,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_progress_bar_enabled: bool,
    pub premium_subscription_count: Option<u64>,
    pub premium_tier: Option<PremiumTier>,
    pub presences: Vec<Presence>,
    pub public_updates_channel_id: Option<Id<ChannelMarker>>,
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    pub safety_alerts_channel_id: Option<Id<ChannelMarker>>,
    pub splash: Option<ImageHash>,
    pub stage_instances: Vec<StageInstance>,
    pub stickers: Vec<Sticker>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub threads: Vec<Channel>,
    pub unavailable: bool,
    pub vanity_url_code: Option<String>,
    pub verification_level: VerificationLevel,
    pub voice_states: Vec<VoiceState>,
    pub widget_channel_id: Option<Id<ChannelMarker>>,
    pub widget_enabled: Option<bool>,
}

impl From<DatabaseGuild> for LuroGuild {
    fn from(guild: DatabaseGuild) -> Self {
        Self {
            data: Some(LuroGuildData {
                accent_colour: guild.accent_colour.map(|x| x as u32),
                accent_colour_custom: guild.custom_accent_colour.map(|x| x as u32),
            }),
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
            icon: Default::default(),
            joined_at: Default::default(),
            large: Default::default(),
            max_members: Default::default(),
            max_presences: Default::default(),
            max_video_channel_users: Default::default(),
            member_count: Default::default(),
            members: Default::default(),
            mfa_level: MfaLevel::from(guild.mfa_level as u8),
            nsfw_level: NSFWLevel::from(guild.nsfw_level as u8),
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
            guild_id: guild.guild_id,
            name: guild.name,
            owner_id: guild.owner_id,
        }
    }
}

impl From<LuroGuild> for Guild {
    fn from(guild: LuroGuild) -> Self {
        Self {
            afk_channel_id: Default::default(),
            afk_timeout: guild.afk_timeout,
            application_id: Default::default(),
            approximate_member_count: Default::default(),
            approximate_presence_count: Default::default(),
            banner: Default::default(),
            channels: Default::default(),
            default_message_notifications: guild.default_message_notifications,
            description: Default::default(),
            discovery_splash: Default::default(),
            emojis: Default::default(),
            explicit_content_filter: guild.explicit_content_filter,
            features: Default::default(),
            icon: Default::default(),
            id: guild.guild_id(),
            joined_at: Default::default(),
            large: Default::default(),
            max_members: Default::default(),
            max_presences: Default::default(),
            max_video_channel_users: Default::default(),
            member_count: Default::default(),
            members: Default::default(),
            mfa_level: guild.mfa_level,
            name: Default::default(),
            nsfw_level: guild.nsfw_level,
            owner_id: Id::new(guild.owner_id as u64),
            owner: Default::default(),
            permissions: Default::default(),
            preferred_locale: Default::default(),
            premium_progress_bar_enabled: Default::default(),
            premium_subscription_count: Default::default(),
            premium_tier: Default::default(),
            presences: Default::default(),
            public_updates_channel_id: Default::default(),
            roles: Default::default(),
            rules_channel_id: Default::default(),
            safety_alerts_channel_id: Default::default(),
            splash: Default::default(),
            stage_instances: Default::default(),
            stickers: Default::default(),
            system_channel_flags: guild.system_channel_flags,
            system_channel_id: Default::default(),
            threads: Default::default(),
            unavailable: Default::default(),
            vanity_url_code: Default::default(),
            verification_level: guild.verification_level,
            voice_states: Default::default(),
            widget_channel_id: Default::default(),
            widget_enabled: Default::default(),
        }
    }
}

impl From<Guild> for LuroGuild {
    fn from(guild: Guild) -> Self {
        Self {
            data: None,
            afk_channel_id: Default::default(),
            afk_timeout: guild.afk_timeout,
            application_id: Default::default(),
            approximate_member_count: Default::default(),
            approximate_presence_count: Default::default(),
            banner: Default::default(),
            channels: Default::default(),
            default_message_notifications: guild.default_message_notifications,
            description: Default::default(),
            discovery_splash: Default::default(),
            emojis: Default::default(),
            explicit_content_filter: guild.explicit_content_filter,
            features: Default::default(),
            icon: Default::default(),
            guild_id: guild.id.get() as i64,
            joined_at: Default::default(),
            large: Default::default(),
            max_members: Default::default(),
            max_presences: Default::default(),
            max_video_channel_users: Default::default(),
            member_count: Default::default(),
            members: Default::default(),
            mfa_level: guild.mfa_level,
            name: Default::default(),
            nsfw_level: guild.nsfw_level,
            owner_id: guild.owner_id.get() as i64,
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
            system_channel_flags: guild.system_channel_flags,
            system_channel_id: Default::default(),
            threads: Default::default(),
            unavailable: Default::default(),
            vanity_url_code: Default::default(),
            verification_level: guild.verification_level,
            voice_states: Default::default(),
            widget_channel_id: Default::default(),
            widget_enabled: Default::default(),
        }
    }
}