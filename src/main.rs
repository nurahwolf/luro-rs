#![feature(let_chains)]

use anyhow::Context;
use futures_util::StreamExt;
use interactions::InteractionResponse;
use models::LuroFramework;
use std::{env, sync::Arc};
use tracing_subscriber::{filter, fmt, prelude::__tracing_subscriber_SubscriberExt, reload, util::SubscriberInitExt};
use twilight_gateway::{stream::ShardEventStream, Intents};

pub mod commands;
pub mod error;
pub mod event_handler;
pub mod functions;
pub mod hecks;
pub mod interactions;
pub mod macros;
pub mod models;
pub mod responses;

/// [tracing_subscriber] filter level
pub const FILTER: filter::LevelFilter = filter::LevelFilter::INFO;

/// Luro's main accent colour
pub const ACCENT_COLOUR: u32 = 0xDABEEF;
// Luro's primary owner
pub const BOT_OWNER: u64 = 373524896187416576;
/// Luro's DANGER colour
pub const COLOUR_DANGER: u32 = 0xD35F5F;
/// Transparent embed color (dark theme)
pub const COLOUR_TRANSPARENT: u32 = 0x2F3136;
/// Luro's SUCCESS colour
pub const COLOUR_SUCCESS: u32 = 0xA0D995;
/// The core data directory for Luro. By default this is at the "data" folder within Luro.
/// Consider setting this to XDG_DATA_HOME on a production system.
pub const DATA_PATH: &str = "data/";
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

/// Regex for matching content within code blocks
pub const REGEX_CODE_BLOCK: &str = r"\`\`\`s\n?([\s\S]*?)\n?\`\`\`|\`\`\`\n?([\s\S]*?)\n?\`\`\`";

// TYPES
/// A shorthand to [LuroFramework] wrapped in an [Arc].
pub type LuroContext = Arc<LuroFramework>;
/// Luro's response type for interactions, returning an [InteractionResponse] if successful, or  [anyhow::Error] if unsuccessful
pub type SlashResponse = Result<InteractionResponse, anyhow::Error>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise the tracing subscriber
    let (filter, reload_handle) = reload::Layer::new(FILTER);
    tracing_subscriber::registry().with(filter).with(fmt::Layer::default()).init();

    // Key things needed for the framework
    let (token, lavalink_host, lavalink_auth, intents) = (
        env::var("DISCORD_TOKEN").context("Failed to get the variable DISCORD_TOKEN")?,
        env::var("LAVALINK_HOST").context("Failed to get the variable LAVALINK_HOST")?,
        env::var("LAVALINK_AUTHORISATION").context("Failed to get the variable LAVALINK_AUTHORISATION")?,
        Intents::all()
    );
    // Create the framework
    let (luro, mut shards) = LuroFramework::builder(intents, lavalink_auth, lavalink_host, token, reload_handle).await?;
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

        tokio::spawn(LuroFramework::handle_event(luro.clone(), event, shard.sender()));
    }

    Ok(())
}
