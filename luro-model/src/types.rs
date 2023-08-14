use dashmap::DashMap;
use twilight_model::{
    application::interaction::Interaction,
    id::{
        marker::{GuildMarker, UserMarker},
        Id
    }
};

use crate::{guild_setting::GuildSetting, heck::Heck, luro_message::LuroMessage, luro_user::LuroUser, story::Story};

/// A simple wrapper around stories. Primary key is the ID of the story.
#[cfg(not(feature = "toml-driver"))]
pub type Stories = DashMap<usize, Story>;
#[cfg(feature = "toml-driver")]
pub type Stories = DashMap<String, Story>;

/// A [DashMap] containing a [Heck], with an index of the heck ID
#[cfg(not(feature = "toml-driver"))]
pub type Hecks = DashMap<usize, Heck>;
#[cfg(feature = "toml-driver")]
pub type Hecks = DashMap<String, Heck>;

/// A [DashMap] containing guild specific settings ([GuildSetting]), keyed by [GuildMarker].
pub type GuildData = DashMap<Id<GuildMarker>, GuildSetting>;

/// A [DashMap] containing user specific settings ([LuroUser]), keyed by [UserMarker].
pub type LuroUserData = DashMap<Id<UserMarker>, LuroUser>;

/// A [DashMap] containing an [Interaction], keyed by a [String]. Generally the message ID, but can be other markers too. This is primarily used for recalling interactions in the future
pub type CommandManager = DashMap<String, Interaction>;

/// A simple wrapper around quotes. Primary key is the ID of the story.
#[cfg(not(feature = "toml-driver"))]
pub type Quotes = DashMap<usize, LuroMessage>;
#[cfg(feature = "toml-driver")]
pub type Quotes = DashMap<String, LuroMessage>;
