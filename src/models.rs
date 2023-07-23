use std::collections::HashMap;

use hyper::client::HttpConnector;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{reload::Handle, Registry};
use twilight_cache_inmemory::InMemoryCache;

use twilight_http::Client;
use twilight_lavalink::Lavalink;
use twilight_model::{
    application::command::Command,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, UserMarker},
        Id
    },
    oauth::Application,
    user::CurrentUser
};

mod guildsettings;
mod luroframework;

/// A simple struct simply containing if a resposne should be ephemeral or deferred
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LuroResponse {
    pub ephemeral: bool,
    pub deferred: bool
}

/// Settings that are stored on disk and meant to be modified by the user
#[derive(Debug)]
pub struct Settings {
    /// The application ID
    pub application_id: Id<ApplicationMarker>
}

/// Data that may be accessed globally, including DMs. Generally not modified by the end user
#[derive(Debug)]

pub struct GlobalData {
    pub application: Application,
    pub count: usize,
    pub current_user: CurrentUser,
    pub hecks: Hecks,
    pub owners: Vec<Id<UserMarker>>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Heck {
    pub heck_message: String,
    pub author_id: u64
}

/// Structure for `heck.toml`
/// We have two hecks, one that is slowly drained (so we only get a heck once) and another used to get explicit hecks.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Hecks {
    /// A vector containing all SFW hecks
    pub sfw_hecks: Vec<Heck>,
    /// A vector containing all NSFW hecks
    pub nsfw_hecks: Vec<Heck>,
    /// A vector of [usize] that contains availalbe random hecks to get. The hecks are reloaded when this reaches zero.
    pub sfw_heck_ids: Vec<usize>,
    /// A vector of [usize] that contains availalbe random hecks to get. The hecks are reloaded when this reaches zero.
    pub nsfw_heck_ids: Vec<usize>
}

/// The framework used to dispatch slash commands.
#[derive(Debug)]
pub struct LuroFramework {
    /// HTTP client used for making outbound API requests
    pub hyper_client: hyper::Client<HttpConnector>,
    /// Lavalink client, for playing music
    pub lavalink: Lavalink,
    /// Twilight's client for interacting with the Discord API
    pub twilight_client: Client,
    /// Twilight's cache
    pub twilight_cache: InMemoryCache,
    /// The global tracing subscriber, for allowing manipulation within commands
    pub tracing_subscriber: Handle<LevelFilter, Registry>,

    /// Settings that are stored on disk and meant to be modified by the user
    pub settings: RwLock<Settings>,
    /// Data that may be accessed globally, including DMs
    pub global_data: RwLock<GlobalData>,
    /// Data that is specific to a guild
    pub guild_data: RwLock<HashMap<Id<GuildMarker>, GuildSetting>>
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GuildSettings {
    /// Guild Settings
    pub guilds: HashMap<Id<GuildMarker>, GuildSetting>
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GuildSetting {
    /// Commands registered to a guild
    pub commands: Vec<Command>,
    /// Private hecks for this specific guild
    pub hecks: Hecks,
    /// Guild Accent Colour, which is the first colour role within a guild
    pub accent_colour: u32,
    /// An administrator may wish to override the colour in which case this is set.
    pub accent_colour_custom: Option<u32>,
    /// Discord events are logged here, if defined
    pub discord_events_log_channel: Option<Id<ChannelMarker>>,
    /// Moderator actions are pushed here such as bans, if defined
    pub moderator_actions_log_channel: Option<Id<ChannelMarker>>
}
