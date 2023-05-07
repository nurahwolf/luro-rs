#![feature(let_chains)]

use futures::StreamExt;
use luro::Luro;
use twilight_gateway::stream::ShardEventStream;

pub mod commands;
pub mod data;
pub mod errors;
pub mod event_handler;
pub mod functions;
pub mod luro;

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
pub const ACCENT_COLOUR: u32 = 0xDABEEF;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise the tracing subscriber.
    tracing_subscriber::fmt::init();

    tracing::info!("Booting Luro!");
    let (luro, mut shards) = Luro::default().await?;

    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((shard, event)) = stream.next().await {
        match event {
            Ok(event) => {
                if let Err(why) = luro.clone().handle_event(event, shard).await {
                    tracing::warn!(?why, "error handling event");
                };
            }
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };
    }

    Ok(())
}
