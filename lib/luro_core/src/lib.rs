use config::Config;
use favorites::Favs;
use heck::Hecks;
use quotes::Quotes;
use secrets::Secrets;
use std::sync::{atomic::AtomicUsize, Arc};
use stories::Stories;
use tokio::sync::RwLock;

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
pub const HECK_FILE_PATH: &str = "data/heck.toml";
/// Where the quotes toml file lives. Can be overriden elsewhere if desired.
pub const QUOTES_FILE_PATH: &str = "data/quotes.toml";
/// Where the user_favs toml file lives. Can be overriden elsewhere if desired.
pub const FAVORITES_FILE_PATH: &str = "data/user_favs.toml";
/// Where the secrets toml file lives. Make sure this is in a safe space and with strong permissions!
pub const SECRETS_FILE_PATH: &str = "data/secrets.toml";
/// Where the stories toml file lives. Can be overriden elsewhere if desired.
pub const STORIES_FILE_PATH: &str = "data/stories.toml";
/// Where the fursona folder lives. Can be overriden elsewhere if desired.
pub const FURSONA_FILE_PATH: &str = "data/fursona";
/// The regex used to match furaffinity posts.
pub const FURAFFINITY_REGEX: &str = r"(?:https://)?(?:www\.)?furaffinity\.net/(?:view|full)/(?P<submission_id>\d+)/?|https://d\.(?:facdn|furaffinity).net/art/(?P<author>[\w\-.~:?#\[\]@!$&'()*+,;=%]+)/(?P<cdn_id>\d+)/(?P<original_cdn_id>\d*).\S*(?:gif|jpe?g|tiff?|png|webp|bmp)";
/// Regex to pull out links from a message, which is then passed to the source finder commands.
pub const SOURCE_FINDER_REGEX: &str = r"(?P<url>http[^\s>]+)";
/// The timeout duriation for command buttons, in seconds.
pub const TIMEOUT_DURIATION: u64 = 12 * 60;

// Types
/// Luro's error type
pub type Error = Box<dyn std::error::Error + Send + Sync>;
/// Luro's context, which allows the user to grab the serenity context + data struct
pub type Context<'a> = poise::Context<'a, Data, Error>;
/// A wrapped around the Poise command context, for ease of use.
pub type Command = poise::Command<Data, Error>;

pub mod config;
pub mod favorites;
pub mod heck;
pub mod quotes;
pub mod secrets;
pub mod stories;

// TODO: Can I have one impl for all of these?
// TODO: Create a baseline template via code (Such as if the toml file does not exist)
// TODO: Commented out functions are not used yet!

/// **Luro's Data**
///
/// A struct holding all user data that makes Luro tick.
///
/// This structure intends to use best practices <https://github.com/serenity-rs/serenity/blob/current/examples/e12_global_data/src/main.rs>, for example `Arc<RwLock<HashMap<String, u64>>>` for read/write, and `Arc<AtomicUsize>` for read only data.
pub struct Data {
    /// Configuration that is got from the "config.toml" file. This is intended to be user modifiable and easy, by non-technically inclined users.
    /// NOTE: There is "constants.rs" where a bunch of other 'config' like variables live, however these are intended for ADVANCED USERS, hence they live here.
    pub config: Arc<RwLock<Config>>,
    /// Luro's Database, which is currently a sled.rs instance.
    pub database: Arc<sled::Db>,
    /// Heck: A bunch of silly messages to throw at a user. This refers to the "heck.toml" file on disk.
    pub heck: Arc<RwLock<Hecks>>,
    /// Quotes: A bunch of silly messages that people have said. This refers to the "quotes.toml" file on disk.
    pub quotes: Arc<RwLock<Quotes>>,
    /// User Favs: Messages that a user has favorited. This refers to the "user_favs.toml" file on disk.
    pub user_favorites: Arc<RwLock<Favs>>,
    /// Application secrets got from the "secrets.toml" file on disk.
    pub secrets: Arc<secrets::Secrets>,
    /// Stories: A bunch of 'stories', which are more shitposty in nature. This refers to the "stories.toml" file on disk.
    pub stories: Arc<RwLock<Stories>>,
    /// A Songbird instance for voice fun.
    pub songbird: Arc<songbird::Songbird>,
    /// The total commands that have been ran in this instance. NOTE: This is RESET when the bot restarts! It only lives in memory.
    pub command_total: Arc<RwLock<AtomicUsize>>
}

pub async fn initialise_data() -> Data {
    let database = match sled::open(DATABASE_FILE_PATH) {
        Ok(db) => db,
        Err(err) => panic!("Failed to open / create database at the path: {DATABASE_FILE_PATH}\nReason: {err}")
    };

    Data {
        config: RwLock::new(Config::get(CONFIG_FILE_PATH).await).into(),
        database: database.into(),
        heck: RwLock::new(Hecks::get(HECK_FILE_PATH).await).into(),
        quotes: RwLock::new(Quotes::get(QUOTES_FILE_PATH).await).into(),
        user_favorites: RwLock::new(Favs::get(FAVORITES_FILE_PATH).await).into(),
        secrets: Secrets::get(SECRETS_FILE_PATH).await.into(),
        stories: RwLock::new(Stories::get(STORIES_FILE_PATH).await).into(),
        songbird: songbird::Songbird::serenity(),
        command_total: RwLock::new(AtomicUsize::new(0)).into() // NOTE: Resets to zero on bot restart, by design
    }
}
