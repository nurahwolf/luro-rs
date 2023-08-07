use dashmap::DashMap;
use twilight_model::{
    application::interaction::Interaction,
    id::{
        marker::{GuildMarker, MessageMarker, UserMarker},
        Id
    }
};

use crate::{guild_setting::GuildSetting, heck::Heck, luro_user::LuroUser, story::Story};

/// A simple wrapper around stories. Primary key is the ID of the story.
pub type Stories = DashMap<usize, Story>;

/// A [DashMap] containing a [Heck], with an index of the heck ID
pub type Hecks = DashMap<usize, Heck>;

/// A [DashMap] containing guild specific settings ([GuildSetting]), keyed by [GuildMarker].
pub type GuildData = DashMap<Id<GuildMarker>, GuildSetting>;

/// A [DashMap] containing user specific settings ([LuroUser]), keyed by [UserMarker].
pub type LuroUserData = DashMap<Id<UserMarker>, LuroUser>;

/// A [DashMap] containing an [Interaction], keyed by [MessageMarker]. This is primarily used for recalling interactions in the future
pub type CommandManager = DashMap<Id<MessageMarker>, Interaction>;
