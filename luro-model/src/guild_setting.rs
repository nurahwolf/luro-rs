use serde::{Deserialize, Serialize};
use twilight_model::{
    application::command::Command,
    id::{
        marker::{ChannelMarker, RoleMarker},
        Id
    }
};

use crate::{
    functions::{deserialize_heck_id, serialize_heck_id},
    types::Hecks
};

/// Settings that are specific to a guild
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GuildSetting {
    #[cfg(feature = "toml-driver")]
    #[serde(deserialize_with = "deserialize_heck_id", serialize_with = "serialize_heck_id", default)]
    pub available_random_nsfw_hecks: Vec<usize>,
    #[cfg(not(feature = "toml-driver"))]
    pub available_random_nsfw_hecks: Vec<usize>,
    #[cfg(feature = "toml-driver")]
    #[serde(deserialize_with = "deserialize_heck_id", serialize_with = "serialize_heck_id", default)]
    pub available_random_sfw_hecks: Vec<usize>,
    #[cfg(not(feature = "toml-driver"))]
    pub available_random_sfw_hecks: Vec<usize>,
    /// The Guild's name
    pub guild_name: String,
    /// Commands registered to a guild
    pub commands: Vec<Command>,
    /// Private NSFW hecks for this specific guild
    #[serde(default)]
    pub nsfw_hecks: Hecks,
    /// Private SFW hecks for this specific guild
    #[serde(default)]
    pub sfw_hecks: Hecks,
    /// Guild Accent Colour, which is the first colour role within a guild
    pub accent_colour: u32,
    /// An administrator may wish to override the colour in which case this is set.
    pub accent_colour_custom: Option<u32>,
    /// Log ALL events here, unless you set more specific channels
    pub catchall_log_channel: Option<Id<ChannelMarker>>,
    /// Events relating to threads (Create, modify, Delete) are logged here
    pub thread_events_log_channel: Option<Id<ChannelMarker>>,
    /// Events relating to messages (Create, modify, Delete) are logged here
    pub message_events_log_channel: Option<Id<ChannelMarker>>,
    /// Events relating to moderation (Ban, Kick) are logged here
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>,
    #[serde(default)]
    /// Optional roles to disallow in the self assignable roles module
    pub assignable_role_blacklist: Vec<Id<RoleMarker>>
}
