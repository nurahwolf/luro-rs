use serde::{Deserialize, Serialize};
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use super::actions_type::UserActionType;

/// Bans recorded against a user
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize,)]
pub struct UserActions {
    /// The type of action this is
    pub action_type: Vec<UserActionType,>,
    /// The guild that the action took place
    pub guild_id: Option<Id<GuildMarker,>,>,
    /// The reason that the action took place
    pub reason: Option<String,>,
    /// Who performed this action
    pub responsible_user: Id<UserMarker,>,
}
