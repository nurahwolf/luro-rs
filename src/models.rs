use std::collections::{BTreeMap, HashMap};

use hyper::client::HttpConnector;
use parking_lot::RwLock;
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
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
    user::{CurrentUser, User}
};

pub mod global_data;
mod guildsettings;
mod hecks;
mod luroframework;
pub mod toml;
pub mod user_data;

/// Settings that are stored on disk and meant to be modified by the user
#[derive(Debug)]
pub struct Settings {
    /// The application ID
    pub application_id: Id<ApplicationMarker>
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Stories {
    pub stories: Vec<Story>
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Story {
    pub title: String,
    pub description: String
}

/// Data that is specific to a user
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserData {
    pub wordcount: usize,
    pub averagesize: usize,
    #[serde(deserialize_with = "deserialize_data", serialize_with = "serialize_data")]
    pub wordsize: BTreeMap<usize, usize>,
    pub words: BTreeMap<String, usize>
}

fn serialize_data<S>(input: &BTreeMap<usize, usize>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer
{
    let data = input
        .iter()
        .map(|(str_key, value)| (str_key.to_string(), *value))
        .collect::<BTreeMap<String, usize>>();

    s.collect_map(data)
}

fn deserialize_data<'de, D>(deserializer: D) -> Result<BTreeMap<usize, usize>, D::Error>
where
    D: Deserializer<'de>
{
    let str_map = BTreeMap::<String, usize>::deserialize(deserializer)?;
    let original_len = str_map.len();
    let data = {
        str_map
            .into_iter()
            .map(|(str_key, value)| match str_key.parse() {
                Ok(int_key) => Ok((int_key, value)),
                Err(_) => Err(de::Error::invalid_value(
                    de::Unexpected::Str(&str_key),
                    &"a non-negative integer"
                ))
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?
    };
    // multiple strings could parse to the same int, e.g "0" and "00"
    if data.len() < original_len {
        return Err(de::Error::custom("detected duplicate integer key"));
    }
    Ok(data)
}

/// Data that may be accessed globally, including DMs. Generally not modified by the end user
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GlobalData {
    pub application: Application,
    pub count: usize,
    pub current_user: CurrentUser,
    pub hecks: Hecks,
    pub stories: Vec<Story>,
    pub owners: Vec<User>
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
    pub guild_data: RwLock<HashMap<Id<GuildMarker>, GuildSetting>>,
    /// Data that is specific to a user
    pub user_data: RwLock<HashMap<Id<UserMarker>, UserData>>
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct GuildSetting {
    /// The Guild's name
    pub guild_name: String,
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
