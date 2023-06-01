use core::fmt;
use std::{sync::RwLock, collections::HashMap};

use hyper::client::HttpConnector;
use twilight_cache_inmemory::InMemoryCache;
use twilight_lavalink::Lavalink;
use twilight_model::id::Id;
use zephyrus::{twilight_exports::{GuildMarker, UserMarker}, command::Command};

/// Luro's data, containin all the fun things needed to work!
pub struct LuroData {
    /// Twilight cache, for useful cached things!
    pub twilight_cache: InMemoryCache,
    /// Lavalink client, for playing music
    pub lavalink: Lavalink,
    /// Luro's direct HTTP client, for making API requests and interacting with lavalink
    pub hyper_client: hyper::Client<HttpConnector>,
    /// A hashmap containing the number of commands run. The entry `total` includes the amount of all commands run.
    pub command_usage: RwLock<HashMap<String, usize>>,
    /// A hashmap containing ettings per guild
    pub guild_settings: RwLock<HashMap<Id<GuildMarker>, GuildSettings>>,
    /// A hashmap containing settings per user
    pub user_settings: RwLock<HashMap<Id<UserMarker>, UserSettings>>,
    /// Data for commands that are spread cross-server
    pub command_data: GlobalCommands,
}

/// Guild specific data
pub struct GuildSettings {
    /// A vector of commands that are loaded in a guild
    pub guild_commands: RwLock<Vec<Command>>,
}

pub struct UserSettings {

}

/// Data for global commands
pub struct GlobalCommands {
    /// Global hecks! Woo
    pub global_hecks: RwLock<Hecks>,
}

#[derive(Debug)]
pub enum LuroError {
    NoInteractionData,
    NoApplicationCommand,
    NoMessageInteractionData,
    NoApplicationData,
}

impl std::error::Error for LuroError {}

impl fmt::Display for LuroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LuroError::NoMessageInteractionData => write!(f, "No Message Interaction Data"),
            LuroError::NoInteractionData => write!(f, "No data was found in the interaction"),
            LuroError::NoApplicationCommand => write!(
                f,
                "No ApplicationCommand was found in the interaction's data"
            ),
            LuroError::NoApplicationData => write!(f, "Unable to get data from the application rwlock")
            // _ => write!(f, "Error description not written yet"),
        }
    }
}