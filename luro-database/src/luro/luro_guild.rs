use luro_model::role::LuroRole;
use serde::{Deserialize, Serialize};
use twilight_model::{
    channel::{message::Sticker, Channel, StageInstance},
    gateway::presence::Presence,
    guild::{
        AfkTimeout, DefaultMessageNotificationLevel, Emoji, ExplicitContentFilter, GuildFeature, MfaLevel, NSFWLevel, Permissions,
        PremiumTier, SystemChannelFlags, VerificationLevel,
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, UserMarker},
        Id,
    },
    util::{ImageHash, Timestamp},
    voice::VoiceState,
};

mod alert_channels;
mod guild_id;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LuroGuild {
    /// The accent colour of the guild. This is calculated by the first role in a guild that has a colour
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_colour: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: AfkTimeout,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catchall_log_channel: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<Channel>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub everyone_role: Option<LuroRole>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<GuildFeature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nsfw_level: Option<NSFWLevel>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safety_alerts_channel_id: Option<Id<ChannelMarker>>,
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
    pub widget_enabled: Option<bool>,
    /// If the guild has a custom accent set by a member of staff
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_accent_colour: Option<u32>,
    pub guild_id: i64,
    pub name: String,
    pub owner_id: i64,
}
