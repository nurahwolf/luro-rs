#![feature(let_chains)]

use commands::LuroCommands;
use config::{Hecks, LuroGuilds};
use futures::StreamExt;
use hyper::client::{Client as HyperClient, HttpConnector};
use tracing::warn;

use std::sync::Arc;
use twilight_gateway::stream::ShardEventStream;
use twilight_http::Client;
use twilight_lavalink::Lavalink;
use twilight_model::{
    id::{marker::ApplicationMarker, Id},
    user::CurrentUser,
};
use twilight_standby::Standby;

pub mod commands;
pub mod config;
pub mod event_handler;
pub mod functions;
pub mod interactions;
pub mod luro;

pub const ACCENT_COLOUR: u32 = 0xDABEEF;

// THESE CONSTANTS ARE INTENDED TO BE MODIFIABLE BY THE USER! THEY SHOULD NOT BREAK THINGS!!
// Please feel free to change them, and if they break in unexpected ways, raise an issue.

/// Used for setting what environment variable Luro listens for. Defaults to "LURO_TOKEN".
pub const BOT_TOKEN: &str = "LURO_TOKEN";
/// The core data directory for Luro. By default this is at the "data" folder within Luro.
/// Consider setting this to XDG_DATA_HOME on a production system.
pub const DATA_PATH: &str = "data/";
/// Where the config toml file lives. Can be overriden elsewhere if desired.
pub const CONFIG_FILE_PATH: &str = "data/config.toml";
/// Where the database folder lives. Can be overriden elsewhere if desired.
pub const DATABASE_FILE_PATH: &str = "data/database";
/// Where the heck toml file lives. Can be overriden elsewhere if desired.
pub const HECK_FILE_PATH: &str = "data/hecks.toml";
/// Where the quotes toml file lives. Can be overriden elsewhere if desired.
pub const QUOTES_FILE_PATH: &str = "data/quotes.toml";
/// Where the user_favs toml file lives. Can be overriden elsewhere if desired.
pub const FAVOURITES_FILE_PATH: &str = "data/user_favs.toml";
/// Where the secrets toml file lives. Make sure this is in a safe space and with strong permissions!
pub const SECRETS_FILE_PATH: &str = "data/secrets.toml";
/// Where the stories toml file lives. Can be overriden elsewhere if desired.
pub const STORIES_FILE_PATH: &str = "data/stories.toml";
/// Where the guild_settings toml file lives. Can be overriden elsewhere if desired.
pub const GUILDSETTINGS_FILE_PATH: &str = "data/guild_settings.toml";
/// Where the fursona folder lives. Can be overriden elsewhere if desired.
pub const FURSONA_FILE_PATH: &str = "data/fursona";
/// The regex used to match furaffinity posts.
pub const FURAFFINITY_REGEX: &str = r"(?:https://)?(?:www\.)?furaffinity\.net/(?:view|full)/(?P<submission_id>\d+)/?|https://d\.(?:facdn|furaffinity).net/art/(?P<author>[\w\-.~:?#\[\]@!$&'()*+,;=%]+)/(?P<cdn_id>\d+)/(?P<original_cdn_id>\d*).\S*(?:gif|jpe?g|tiff?|png|webp|bmp)";
/// Regex to pull out links from a message, which is then passed to the source finder commands.
pub const SOURCE_FINDER_REGEX: &str = r"(?P<url>http[^\s>]+)";
/// The timeout duriation for command buttons, in seconds.
pub const TIMEOUT_DURIATION: u64 = 12 * 60;

/// Luro: Hello World!
pub struct Luro {
    pub application_id: Id<ApplicationMarker>,
    pub http: Client,
    pub lavalink: Lavalink,
    pub hyper: HyperClient<HttpConnector>,
    pub user: CurrentUser,
    pub standby: Standby,
    pub commands: std::sync::RwLock<LuroCommands>, 
    pub guild_settings: tokio::sync::RwLock<LuroGuilds>,
    pub hecks: tokio::sync::RwLock<Hecks>,
    
    // Global Vars
    interaction_count: tokio::sync::RwLock<u32>,   
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise the tracing subscriber.
    tracing_subscriber::fmt::init();

    // Initialise Luro
    let (luro, mut shards) = match Luro::init().await {
        Ok(ok) => ok,
        Err(err) => panic!("Failed to create Luro! {err}"),
    };

    // Run the primary event loop per shard
    let mut events = ShardEventStream::new(shards.iter_mut());

    loop {
        // Await our event
        let (shard, event) = match events.next().await {
            Some(ok) => ok,
            None => {
                warn!("Event loop just returned no event, somehow...");
                continue;
            }
        };

        // Decode or event
        let event = match event {
            Ok(event) => event,
            Err(why) => {
                warn!("Failed to decode event: {why}");
                continue;
            }
        };

        // Clone Luro, in order to pass through our context. Then create our sender reference
        let luro = Arc::clone(&luro);
        let sender = Arc::new(shard.sender());

        // Now spawn a task to handle the event
        tokio::spawn(async move {
            match Luro::event_handler(luro, sender, event).await {
                Ok(ok) => ok,
                Err(why) => warn!("Failed to spawn a task to handle this event: {why}"),
            };
        });
    }
}
