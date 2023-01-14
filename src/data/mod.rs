use std::sync::{Arc, atomic::AtomicUsize};
use tokio::sync::RwLock;


pub mod config;
pub mod stories;
pub mod heck;
pub mod secrets;
pub mod quotes;

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
    pub config: Arc<RwLock<config::Config>>,
    /// Luro's Database, which is currently a sled.rs instance.
    pub database: Arc<sled::Db>,
    /// Heck: A bunch of silly messages to throw at a user. This refers to the "heck.toml" file on disk.
    pub heck: Arc<RwLock<heck::Heck>>,
    /// Quotes: A bunch of silly messages that people have said. This refers to the "quotes.toml" file on disk.
    pub quotes: Arc<RwLock<quotes::Quotes>>,
    /// Application secrets got from the "secrets.toml" file on disk.
    pub secrets: Arc<secrets::Secrets>,
    /// Stories: A bunch of 'stories', which are more shitposty in nature. This refers to the "stories.toml" file on disk.
    pub stories: Arc<RwLock<stories::Stories>>,
    /// A Songbird instance for voice fun.
    pub songbird: Arc<songbird::Songbird>,
    /// The total commands that have been ran in this instance. NOTE: This is RESET when the bot restarts! It only lives in memory.
    pub command_total: Arc<RwLock<AtomicUsize>>
}