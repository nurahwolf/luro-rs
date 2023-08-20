#![feature(let_chains)]
#![feature(async_fn_in_trait)]

use anyhow::Context;
use dotenv::dotenv;
use framework::Framework;
use futures_util::StreamExt;
use luro_database::TomlDatabaseDriver;
use std::{env, sync::Arc};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{
    fmt,
    prelude::__tracing_subscriber_SubscriberExt,
    reload::{self, Layer},
    util::SubscriberInitExt,
    Registry
};
use twilight_gateway::{error::ReceiveMessageErrorType, stream::ShardEventStream, Intents};

pub mod commands;
pub mod event_handler;
pub mod framework;
pub mod functions;
pub mod interaction;
pub mod interactions;
pub mod luro_command;
pub mod models;
pub mod responses;

/// Where the config toml file lives. Can be overriden elsewhere if desired.
pub const CONFIG_FILE_PATH: &str = "data/config.toml";
/// Where the database folder lives. Can be overriden elsewhere if desired.
pub const DATABASE_FILE_PATH: &str = "data/database";
/// Where the heck toml file lives. Can be overriden elsewhere if desired.
pub const HECK_FILE_PATH: &str = "data/heck.toml";
/// Where the quotes toml file lives. Can be overriden elsewhere if desired.
pub const QUOTES_FILE_PATH: &str = "data/quotes.toml";
/// Where the user_favs toml file lives. Can be overriden elsewhere if desired.
pub const FAVOURITES_FILE_PATH: &str = "data/user_favs.toml";
/// Where the secrets toml file lives. Make sure this is in a safe space and with strong permissions!
pub const SECRETS_FILE_PATH: &str = "data/secrets.toml";
/// Where the stories toml file lives. Can be overriden elsewhere if desired.
pub const STORIES_FILE_PATH: &str = "data/stories.toml";
/// A folder where <guild/guild_id.toml> are stored
pub const GUILDSETTINGS_FILE_PATH: &str = "data/guilds";
/// A folder where <user/user_id.toml> are stored
pub const USERDATA_FILE_PATH: &str = "data/user";
/// Where the fursona folder lives. Can be overriden elsewhere if desired.
pub const FURSONA_FILE_PATH: &str = "data/fursona";
/// The timeout duriation for command buttons, in seconds.
pub const TIMEOUT_DURIATION: u64 = 12 * 60;

// REGEX constants
// =========
/// Regex for matching content within code blocks
pub const REGEX_CODE_BLOCK: &str = r"\`\`\`s\n?([\s\S]*?)\n?\`\`\`|\`\`\`\n?([\s\S]*?)\n?\`\`\`";
/// The regex used to match furaffinity posts.
pub const FURAFFINITY_REGEX: &str = r"(?:https://)?(?:www\.)?furaffinity\.net/(?:view|full)/(?P<submission_id>\d+)/?|https://d\.(?:facdn|furaffinity).net/art/(?P<author>[\w\-.~:?#\[\]@!$&'()*+,;=%]+)/(?P<cdn_id>\d+)/(?P<original_cdn_id>\d*).\S*(?:gif|jpe?g|tiff?|png|webp|bmp)";
/// Regex to pull out links from a message, which is then passed to the source finder commands.
pub const SOURCE_FINDER_REGEX: &str = r"(?P<url>http[^\s>]+)";

// SETTINGS constants
// =========
/// The name of the bot
pub const BOT_NAME: &str = "luro";
/// The name of the bot in lowercase

/// [tracing_subscriber] filter level
pub const FILTER: LevelFilter = LevelFilter::INFO;
// Luro's intents. Can be set to all, but rather spammy.
pub const INTENTS: Intents = Intents::all();
/// Luro's main accent colour
pub const ACCENT_COLOUR: u32 = 0xDABEEF;
/// Luro's DANGER colour
pub const COLOUR_DANGER: u32 = 0xD35F5F;
/// Transparent embed color (dark theme)
pub const COLOUR_TRANSPARENT: u32 = 0x2F3136;
/// Luro's SUCCESS colour
pub const COLOUR_SUCCESS: u32 = 0xA0D995;
/// The name used for Luro's webhooks
pub const WEBHOOK_NAME: &str = "LuroHook";

// PATH constants
// =========
/// The core data directory for Luro. By default this is at the "data" folder within Luro.
/// Consider setting this to XDG_DATA_HOME on a production system.
pub const DATA_PATH: &str = "data/";
/// The log path. By default this is a sub directory of DATA_PATH
pub const LOG_PATH: &str = "data/log/";

/// A shorthand to [Framework] wrapped in an [Arc].
pub type LuroFramework = Arc<Framework<TomlDatabaseDriver>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let driver = TomlDatabaseDriver {};
    let (filter, tracing_subscriber) = reload::Layer::new(FILTER);
    let (token, lavalink_host, lavalink_auth, intents) = (
        env::var("DISCORD_TOKEN").context("Failed to get the variable DISCORD_TOKEN")?,
        env::var("LAVALINK_HOST").context("Failed to get the variable LAVALINK_HOST")?,
        env::var("LAVALINK_AUTHORISATION").context("Failed to get the variable LAVALINK_AUTHORISATION")?,
        INTENTS - Intents::GUILD_PRESENCES
    );

    // Create the framework
    let (luro, mut shards) =
        Framework::builder(driver, intents, lavalink_auth, lavalink_host, token, tracing_subscriber).await?;

    // Initialise tracing for logs
    init_tracing_subscriber(filter, &luro.database.current_user.read().unwrap().name);

    // Start the configured database
    TomlDatabaseDriver::start().await?;

    // Work on our events
    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((shard, event)) = stream.next().await {
        let event = match event {
            Err(error) => {
                if error.is_fatal() {
                    eprintln!("Gateway connection fatally closed, error: {error:?}");
                    break;
                }

                match error.kind() {
                    ReceiveMessageErrorType::Deserializing { event } => {
                        tracing::warn!("Failed to deserialise an object. Check DEBUG for the raw output");
                        tracing::debug!(?event, "error while deserialising event");
                        continue;
                    }
                    _ => {
                        tracing::warn!(?error, "error while receiving event");
                        continue;
                    }
                }
            }
            Ok(event) => event
        };

        tokio::spawn(luro.clone().event_handler(event, shard.sender(), shard.latency().clone()));
    }

    Ok(())
}

fn init_tracing_subscriber(filter: Layer<LevelFilter, Registry>, file_name: &String) {
    let file_appender = tracing_appender::rolling::hourly(LOG_PATH, format!("{file_name}.log"));
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let layer = fmt::layer().with_writer(non_blocking);
    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .with(fmt::Layer::default())
        .init();
}
