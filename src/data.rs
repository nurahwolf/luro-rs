use std::collections::HashMap;

use anyhow::Error;
use redis::{AsyncCommands, RedisError};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use twilight_model::{
    application::command::Command,
    id::{
        marker::{ChannelMarker, GuildMarker},
        Id,
    },
};

use crate::luro::Luro;

use self::hecks::{Heck, Hecks};

pub mod config;
pub mod favourites;
pub mod guild_settings;
pub mod hecks;
pub mod quotes;
pub mod secrets;
pub mod stories;

/// Luro's mutable data hold
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

pub async fn get_hecks(luro: Luro) -> Result<Hecks, Error> {
    let mut con = luro.redis_connection;
    let sfw_hecks = vec![
        Heck {
            heck_message: "sfw owo!".to_string(),
            author_id: 97003404601094144,
        },
        Heck {
            heck_message: "sfw uwu!".to_string(),
            author_id: 97003404601094144,
        },
    ];
    let nsfw_hecks = vec![
        Heck {
            heck_message: "nsfw hello!".to_string(),
            author_id: 97003404601094144,
        },
        Heck {
            heck_message: "nsfw hi!".to_string(),
            author_id: 97003404601094144,
        },
    ];
    let hecks = Hecks {
        sfw_hecks,
        nsfw_hecks,
        sfw_heck_ids: vec![1, 2],
        nsfw_heck_ids: vec![1, 2],
    };

    // Save hecks
    let heck_bincode = bincode::serialize(&hecks)?;
    con.set("hecks", heck_bincode).await?;

    // New get hecks
    let bin: Vec<u8> = con.get("hecks").await?;
    let hecks_db: Hecks = bincode::deserialize(&bin)?;

    Ok(hecks_db)
}

impl Luro {
    pub async fn get_db() -> Result<redis::Client, RedisError> {
        redis::Client::open("redis://127.0.0.1/")
    }
}
