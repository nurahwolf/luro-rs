use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use twilight_model::{
    application::command::Command,
    channel::{message::Sticker, Channel, StageInstance},
    gateway::presence::Presence,
    guild::{
        AfkTimeout, DefaultMessageNotificationLevel, Emoji, ExplicitContentFilter, GuildFeature, MfaLevel, NSFWLevel,
        Permissions, PremiumTier, SystemChannelFlags, VerificationLevel, Guild
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id
    },
    util::{ImageHash, Timestamp},
    voice::VoiceState
};

use crate::constants::PRIMARY_BOT_OWNER;

use super::{
    heck::Hecks,
    role::{LuroRolePositions, LuroRoles}
};

#[derive(Debug, Clone, Deserialize, Serialize)]

pub struct LuroGuild {
    /// If the guild has a custom accent set by a member of staff
    pub accent_colour_custom: Option<u32>,
    /// The accent colour of the guild. This is calculated by the first role in a guild that has a colour
    pub accent_colour: u32,
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: AfkTimeout,
    pub application_id: Option<Id<ApplicationMarker>>,
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub assignable_role_blacklist: Vec<Id<RoleMarker>>,
    pub banner: Option<ImageHash>,
    pub catchall_log_channel: Option<Id<ChannelMarker>>,
    pub channels: Vec<Channel>,
    pub commands: Vec<Command>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<ImageHash>,
    pub emojis: Vec<Emoji>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<GuildFeature>,
    pub icon: Option<ImageHash>,
    pub id: Id<GuildMarker>,
    pub joined_at: Option<Timestamp>,
    pub large: bool,
    pub max_members: Option<u64>,
    pub max_presences: Option<u64>,
    pub max_video_channel_users: Option<u64>,
    pub member_count: Option<u64>,
    pub members: Vec<Id<UserMarker>>,
    pub message_events_log_channel: Option<Id<ChannelMarker>>,
    pub mfa_level: MfaLevel,
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>,
    pub name: String,
    pub nsfw_hecks: Hecks,
    pub nsfw_level: Option<NSFWLevel>,
    pub owner_id: Id<UserMarker>,
    pub owner: Option<bool>,
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    pub premium_progress_bar_enabled: bool,
    pub premium_subscription_count: Option<u64>,
    pub premium_tier: PremiumTier,
    pub presences: Vec<Presence>,
    pub public_updates_channel_id: Option<Id<ChannelMarker>>,
    pub roles: LuroRoles,
    pub role_positions: LuroRolePositions,
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    pub safety_alerts_channel_id: Option<Id<ChannelMarker>>,
    pub sfw_hecks: Hecks,
    pub splash: Option<ImageHash>,
    pub stage_instances: Vec<StageInstance>,
    pub stickers: Vec<Sticker>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub thread_events_log_channel: Option<Id<ChannelMarker>>,
    pub threads: Vec<Channel>,
    pub unavailable: bool,
    pub vanity_url_code: Option<String>,
    pub verification_level: VerificationLevel,
    pub voice_states: Vec<VoiceState>,
    pub widget_channel_id: Option<Id<ChannelMarker>>,
    pub widget_enabled: Option<bool>
}

impl From<Guild> for LuroGuild {
    fn from(guild: Guild) -> Self {
        let mut members = vec![];
        let mut role_positions = BTreeMap::new();
        let mut roles = BTreeMap::new();

        for role in guild.roles {
            role_positions.insert(role.position as usize, role.id);
            roles.insert(role.id, role.into());
        }

        for member in guild.members {
            members.push(member.user.id)
        }

        Self {
            accent_colour_custom: None,
            // TODO: This
            accent_colour: Default::default(),
            afk_channel_id: guild.afk_channel_id,
            afk_timeout: guild.afk_timeout,
            application_id: guild.application_id,
            approximate_member_count: guild.approximate_member_count,
            approximate_presence_count: guild.approximate_presence_count,
            assignable_role_blacklist: Default::default(),
            banner: guild.banner,
            catchall_log_channel: None,
            channels: guild.channels,
            commands: Default::default(),
            default_message_notifications: guild.default_message_notifications,
            description: guild.description,
            discovery_splash: guild.discovery_splash,
            emojis: guild.emojis,
            explicit_content_filter: guild.explicit_content_filter,
            features: guild.features,
            icon: guild.icon,
            id: guild.id,
            joined_at: guild.joined_at,
            large: guild.large,
            max_members: guild.max_members,
            max_presences: guild.max_presences,
            max_video_channel_users: guild.max_video_channel_users,
            member_count: guild.member_count,
            members,
            message_events_log_channel: None,
            mfa_level: guild.mfa_level,
            moderator_actions_log_channel: None,
            name: guild.name,
            nsfw_hecks: Default::default(),
            nsfw_level: Default::default(),
            owner_id: guild.owner_id,
            owner: guild.owner,
            permissions: guild.permissions,
            preferred_locale: guild.preferred_locale,
            premium_progress_bar_enabled: guild.premium_progress_bar_enabled,
            premium_subscription_count: guild.premium_subscription_count,
            premium_tier: guild.premium_tier,
            presences: guild.presences,
            public_updates_channel_id: guild.afk_channel_id,
            roles,
            role_positions,
            rules_channel_id: guild.afk_channel_id,
            safety_alerts_channel_id: guild.afk_channel_id,
            sfw_hecks: Default::default(),
            splash: guild.splash,
            stage_instances: guild.stage_instances,
            stickers: guild.stickers,
            system_channel_flags: guild.system_channel_flags,
            system_channel_id: guild.afk_channel_id,
            thread_events_log_channel: guild.afk_channel_id,
            threads: guild.threads,
            unavailable: guild.unavailable,
            vanity_url_code: guild.vanity_url_code,
            verification_level: guild.verification_level,
            voice_states: guild.voice_states,
            widget_channel_id: guild.afk_channel_id,
            widget_enabled: guild.widget_enabled
        }
    }
}

impl Default for LuroGuild {
    fn default() -> Self {
        Self {
            accent_colour_custom: Default::default(),
            accent_colour: Default::default(),
            afk_channel_id: Default::default(),
            afk_timeout: AfkTimeout::ONE_HOUR,
            application_id: Default::default(),
            approximate_member_count: Default::default(),
            approximate_presence_count: Default::default(),
            assignable_role_blacklist: Default::default(),
            banner: Default::default(),
            catchall_log_channel: Default::default(),
            channels: Default::default(),
            commands: Default::default(),
            default_message_notifications: DefaultMessageNotificationLevel::All,
            description: Default::default(),
            discovery_splash: Default::default(),
            emojis: Default::default(),
            explicit_content_filter: ExplicitContentFilter::None,
            features: Default::default(),
            icon: Default::default(),
            id: Id::new(0),
            joined_at: Default::default(),
            large: Default::default(),
            max_members: Default::default(),
            max_presences: Default::default(),
            max_video_channel_users: Default::default(),
            member_count: Default::default(),
            members: Default::default(),
            message_events_log_channel: Default::default(),
            mfa_level: MfaLevel::None,
            moderator_actions_log_channel: Default::default(),
            name: Default::default(),
            nsfw_hecks: Default::default(),
            nsfw_level: Default::default(),
            owner_id: PRIMARY_BOT_OWNER,
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
            sfw_hecks: Default::default(),
            splash: Default::default(),
            stage_instances: Default::default(),
            stickers: Default::default(),
            system_channel_flags: SystemChannelFlags::empty(),
            system_channel_id: Default::default(),
            thread_events_log_channel: Default::default(),
            threads: Default::default(),
            unavailable: Default::default(),
            vanity_url_code: Default::default(),
            verification_level: VerificationLevel::None,
            voice_states: Default::default(),
            widget_channel_id: Default::default(),
            widget_enabled: Default::default(),
            role_positions: Default::default()
        }
    }
}
