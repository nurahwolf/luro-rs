use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::{GuildMarker, RoleMarker, ChannelMarker}, Id};

/// Data that is only present when fetched from the database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildData {
    pub accent_colour: Option<u32>,
    pub accent_colour_custom: Option<u32>,
    pub guild_id: Id<GuildMarker>,
    /// Roles blacklistex from self assign
    pub role_blacklist: Vec<Id<RoleMarker>>,
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>
}
