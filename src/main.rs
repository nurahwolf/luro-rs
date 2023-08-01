#![feature(let_chains)]
#![feature(map_entry_replace)]
#![feature(entry_insert)]

use anyhow::Context;
use dotenv::dotenv;
use futures_util::StreamExt;
use models::LuroFramework;
use std::{env, sync::Arc};
use tracing::metadata::LevelFilter;
use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, reload, util::SubscriberInitExt};
use twilight_gateway::{stream::ShardEventStream, Intents};
use twilight_model::id::{marker::UserMarker, Id};

pub mod commands;
pub mod event_handler;
pub mod functions;
pub mod models;
pub mod responses;
pub mod traits;

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
// Luro's primary owner
pub const BOT_OWNERS: [Id<UserMarker>; 2] = [Id::new(373524896187416576), Id::new(138791390279630849)];
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

// TYPES
// =========
/// A shorthand to [LuroFramework] wrapped in an [Arc].
pub type LuroContext = Arc<LuroFramework>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let (filter, reload_handle) = reload::Layer::new(FILTER);
    let (token, lavalink_host, lavalink_auth, intents) = (
        env::var("DISCORD_TOKEN").context("Failed to get the variable DISCORD_TOKEN")?,
        env::var("LAVALINK_HOST").context("Failed to get the variable LAVALINK_HOST")?,
        env::var("LAVALINK_AUTHORISATION").context("Failed to get the variable LAVALINK_AUTHORISATION")?,
        INTENTS
    );
    // Create the framework
    let (luro, mut shards) = LuroFramework::builder(intents, lavalink_auth, lavalink_host, token, reload_handle).await?;

    // Initialise the tracing subscriber
    let file_appender =
        tracing_appender::rolling::hourly(LOG_PATH, format!("{}.log", luro.global_data.read().current_user.name));
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let layer = fmt::layer().with_writer(non_blocking);
    tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .with(fmt::Layer::default())
        .init();

    // Work on our events
    let mut stream = ShardEventStream::new(shards.iter_mut());
    while let Some((shard, event)) = stream.next().await {
        let event = match event {
            Err(error) => {
                if error.is_fatal() {
                    eprintln!("Gateway connection fatally closed, error: {error:?}");
                    break;
                }

                tracing::warn!(?error, "error while receiving event");
                continue;
            }
            Ok(event) => event
        };

        tokio::spawn(luro.clone().handle_event(event, shard.sender()));
    }

    Ok(())
}
