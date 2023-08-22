#[cfg(feature = "toml-driver")]
use crate::database::drivers::toml::{
    deserialize_heck::deserialize_heck, deserialize_role_positions::deserialize_role_positions, serialize_heck::serialize_heck,
    serialize_role_positions::serialize_role_positions
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use twilight_model::{
    application::command::Command,
    channel::{message::Sticker, Channel, StageInstance},
    gateway::presence::Presence,
    guild::{
        AfkTimeout, DefaultMessageNotificationLevel, Emoji, ExplicitContentFilter, Guild, GuildFeature, MfaLevel, NSFWLevel,
        Permissions, PremiumTier, SystemChannelFlags, VerificationLevel
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id
    },
    util::{ImageHash, Timestamp},
    voice::VoiceState
};

pub mod log_channel;

use crate::{
    heck::Hecks,
    role::{LuroRolePositions, LuroRoles},
    PRIMARY_BOT_OWNER
};

/// A [HashMap] containing guild specific settings ([LuroGuild]), keyed by [GuildMarker].
pub type LuroGuilds = HashMap<Id<GuildMarker>, LuroGuild>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LuroGuild {
    /// If the guild has a custom accent set by a member of staff
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_colour_custom: Option<u32>,
    /// The accent colour of the guild. This is calculated by the first role in a guild that has a colour
    #[serde(default)]
    pub accent_colour: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<AfkTimeout>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignable_role_blacklist: Vec<Id<RoleMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catchall_log_channel: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<Channel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<Command>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<DefaultMessageNotificationLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery_splash: Option<ImageHash>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub emojis: Vec<Emoji>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<ExplicitContentFilter>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<GuildFeature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,
    #[serde(default = "id")]
    pub id: Id<GuildMarker>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<Timestamp>,
    #[serde(default)]
    pub large: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_members: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_presences: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_video_channel_users: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<Id<UserMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_events_log_channel: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mfa_level: Option<MfaLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>,
    #[serde(default)]
    pub name: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(
        feature = "toml-driver",
        serde(deserialize_with = "deserialize_heck", serialize_with = "serialize_heck")
    )]
    pub nsfw_hecks: Hecks,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw_level: Option<NSFWLevel>,
    #[serde(default = "owner_id")]
    pub owner_id: Id<UserMarker>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    #[serde(default)]
    pub preferred_locale: String,
    #[serde(default)]
    pub premium_progress_bar_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_subscription_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_tier: Option<PremiumTier>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presences: Vec<Presence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_updates_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub roles: LuroRoles,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(
        feature = "toml-driver",
        serde(
            deserialize_with = "deserialize_role_positions",
            serialize_with = "serialize_role_positions"
        )
    )]
    pub role_positions: LuroRolePositions,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[cfg_attr(
        feature = "toml-driver",
        serde(deserialize_with = "deserialize_heck", serialize_with = "serialize_heck")
    )]
    pub sfw_hecks: Hecks,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub splash: Option<ImageHash>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stage_instances: Vec<StageInstance>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub stickers: Vec<Sticker>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<SystemChannelFlags>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_events_log_channel: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threads: Vec<Channel>,
    #[serde(default)]
    pub unavailable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vanity_url_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<VerificationLevel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub voice_states: Vec<VoiceState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub widget_enabled: Option<bool>
}

impl LuroGuild {
    /// Update a [LuroGuild] with settings from a twilight [Guild]
    pub fn update_guild(&mut self, guild: Guild) -> &mut Self {
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
        // TODO: This
        // self.accent_colour: Default::default();
        self.afk_channel_id = guild.afk_channel_id;
        self.afk_timeout = Some(guild.afk_timeout);
        self.application_id = guild.application_id;
        self.approximate_member_count = guild.approximate_member_count;
        self.approximate_presence_count = guild.approximate_presence_count;
        self.banner = guild.banner;
        self.channels = guild.channels;
        self.default_message_notifications = Some(guild.default_message_notifications);
        self.description = guild.description;
        self.discovery_splash = guild.discovery_splash;
        self.emojis = guild.emojis;
        self.explicit_content_filter = Some(guild.explicit_content_filter);
        self.features = guild.features;
        self.icon = guild.icon;
        self.id = guild.id;
        self.joined_at = guild.joined_at;
        self.large = guild.large;
        self.max_members = guild.max_members;
        self.max_presences = guild.max_presences;
        self.max_video_channel_users = guild.max_video_channel_users;
        self.member_count = guild.member_count;
        self.members = members;
        self.mfa_level = Some(guild.mfa_level);
        self.name = guild.name;
        self.owner_id = guild.owner_id;
        self.owner = guild.owner;
        self.permissions = guild.permissions;
        self.preferred_locale = guild.preferred_locale;
        self.premium_progress_bar_enabled = guild.premium_progress_bar_enabled;
        self.premium_subscription_count = guild.premium_subscription_count;
        self.premium_tier = Some(guild.premium_tier);
        self.presences = guild.presences;
        self.public_updates_channel_id = guild.afk_channel_id;
        self.roles = roles;
        self.role_positions = role_positions;
        self.rules_channel_id = guild.afk_channel_id;
        self.safety_alerts_channel_id = guild.afk_channel_id;
        self.splash = guild.splash;
        self.stage_instances = guild.stage_instances;
        self.stickers = guild.stickers;
        self.system_channel_flags = Some(guild.system_channel_flags);
        self.system_channel_id = guild.afk_channel_id;
        self.thread_events_log_channel = guild.afk_channel_id;
        self.threads = guild.threads;
        self.unavailable = guild.unavailable;
        self.vanity_url_code = guild.vanity_url_code;
        self.verification_level = Some(guild.verification_level);
        self.voice_states = guild.voice_states;
        self.widget_channel_id = guild.afk_channel_id;
        self.widget_enabled = guild.widget_enabled;
        self
    }

    pub fn new(id: Id<GuildMarker>) -> Self {
        Self {
            accent_colour_custom: Default::default(),
            accent_colour: Default::default(),
            afk_channel_id: Default::default(),
            afk_timeout: Default::default(),
            application_id: Default::default(),
            approximate_member_count: Default::default(),
            approximate_presence_count: Default::default(),
            assignable_role_blacklist: Default::default(),
            banner: Default::default(),
            catchall_log_channel: Default::default(),
            channels: Default::default(),
            commands: Default::default(),
            default_message_notifications: Default::default(),
            description: Default::default(),
            discovery_splash: Default::default(),
            emojis: Default::default(),
            explicit_content_filter: Default::default(),
            features: Default::default(),
            icon: Default::default(),
            id,
            joined_at: Default::default(),
            large: Default::default(),
            max_members: Default::default(),
            max_presences: Default::default(),
            max_video_channel_users: Default::default(),
            member_count: Default::default(),
            members: Default::default(),
            message_events_log_channel: Default::default(),
            mfa_level: Default::default(),
            moderator_actions_log_channel: Default::default(),
            name: Default::default(),
            nsfw_hecks: Default::default(),
            nsfw_level: Default::default(),
            owner_id: owner_id(),
            owner: Default::default(),
            permissions: Default::default(),
            preferred_locale: Default::default(),
            premium_progress_bar_enabled: Default::default(),
            premium_subscription_count: Default::default(),
            premium_tier: Default::default(),
            presences: Default::default(),
            public_updates_channel_id: Default::default(),
            roles: Default::default(),
            role_positions: Default::default(),
            rules_channel_id: Default::default(),
            safety_alerts_channel_id: Default::default(),
            sfw_hecks: Default::default(),
            splash: Default::default(),
            stage_instances: Default::default(),
            stickers: Default::default(),
            system_channel_flags: Default::default(),
            system_channel_id: Default::default(),
            thread_events_log_channel: Default::default(),
            threads: Default::default(),
            unavailable: Default::default(),
            vanity_url_code: Default::default(),
            verification_level: Default::default(),
            voice_states: Default::default(),
            widget_channel_id: Default::default(),
            widget_enabled: Default::default()
        }
    }
}

impl From<Guild> for LuroGuild {
    fn from(guild: Guild) -> Self {
        let mut luro = Self::default();
        luro.update_guild(guild);
        luro
    }
}

impl Default for LuroGuild {
    fn default() -> Self {
        Self {
            accent_colour_custom: Default::default(),
            accent_colour: Default::default(),
            afk_channel_id: Default::default(),
            afk_timeout: None,
            application_id: Default::default(),
            approximate_member_count: Default::default(),
            approximate_presence_count: Default::default(),
            assignable_role_blacklist: Default::default(),
            banner: Default::default(),
            catchall_log_channel: Default::default(),
            channels: Default::default(),
            commands: Default::default(),
            default_message_notifications: None,
            description: Default::default(),
            discovery_splash: Default::default(),
            emojis: Default::default(),
            explicit_content_filter: None,
            features: Default::default(),
            icon: Default::default(),
            id: Id::new(69),
            joined_at: Default::default(),
            large: Default::default(),
            max_members: Default::default(),
            max_presences: Default::default(),
            max_video_channel_users: Default::default(),
            member_count: Default::default(),
            members: Default::default(),
            message_events_log_channel: Default::default(),
            mfa_level: Some(MfaLevel::None),
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
            system_channel_flags: None,
            system_channel_id: Default::default(),
            thread_events_log_channel: Default::default(),
            threads: Default::default(),
            unavailable: Default::default(),
            vanity_url_code: Default::default(),
            verification_level: None,
            voice_states: Default::default(),
            widget_channel_id: Default::default(),
            widget_enabled: Default::default(),
            role_positions: Default::default()
        }
    }
}

fn id() -> Id<GuildMarker> {
    Id::new(69)
}

fn owner_id() -> Id<UserMarker> {
    PRIMARY_BOT_OWNER
}
