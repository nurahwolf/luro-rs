//! These constants are user facing and used throughout Luro. They are intended to be updated by the end user.

use twilight_model::id::{marker::UserMarker, Id};

/// The primary owner user ID. Used for some defaults, as well as to say who owns the bot. This MUST  be set, even if a group of people own Luro, as its used as a fallback for when data is not tied to a specific user. For example, see [Story].
pub const PRIMARY_BOT_OWNER: Id<UserMarker> = Id::new(373524896187416576);
/// A folder where <guild/guild_id.toml> are stored
pub const GUILDSETTINGS_FILE_PATH: &str = "data/guilds";
/// A folder where <user/user_id.toml> are stored
pub const USERDATA_FILE_PATH: &str = "data/user";
/// Where the heck toml file lives. Can be overriden elsewhere if desired.
pub const SFW_HECK_FILE_PATH: &str = "data/sfw_hecks.toml";
pub const NSFW_HECK_FILE_PATH: &str = "data/nsfw_hecks.toml";
/// Where the stories toml file lives. Can be overriden elsewhere if desired.
pub const SFW_STORIES_FILE_PATH: &str = "data/nsfw_stories.toml";
pub const NSFW_STORIES_FILE_PATH: &str = "data/sfw_stories.toml";
