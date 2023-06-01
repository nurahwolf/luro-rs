use std::collections::HashMap;

use redis::RedisError;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use twilight_model::{
    application::command::Command,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};

use crate::{luro::Luro, models::hecks::Hecks};

pub mod config;
pub mod favourites;
pub mod guild_settings;
pub mod quotes;
pub mod secrets;
pub mod stories;

/// Luro's mutable data hold
#[derive(Debug)]
pub struct LuroData {
    /// Global commands, initially all at startup. Can be modified during reload, but there is no hotreloading yet
    pub global_commands: Vec<Command>,
    /// Guild specific settings!
    pub guild_settings: RwLock<HashMap<Id<GuildMarker>, GuildSettings>>,
    /// Boop the bot!
    pub boop: RwLock<usize>,
    /// Hecks
    pub hecks: RwLock<Hecks>,
}

/// Specific guild settings, controlled by the guild owner / staff
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GuildSettings {
    /// Guild Accent Colour
    pub accent_colour: u32,
    /// User specified accent colour
    pub accent_colour_custom: Option<u32>,
    /// Discord events are logged here, if defined
    pub discord_events_log_channel: Option<Id<ChannelMarker>>,
    /// Moderator actions are pushed here such as bans, if defined
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>,
}

impl Luro {
    pub async fn get_db() -> Result<redis::Client, RedisError> {
        redis::Client::open("redis://127.0.0.1/")
    }
}
