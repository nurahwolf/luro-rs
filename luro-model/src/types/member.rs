use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::MemberFlags,
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
    util::{ImageHash, Timestamp},
};


use super::MemberData;

/// A warpper around [User], with [Member] details if [Id<GuildMarker>] was present on type creation.
/// Details are primarily fetched from the database, but this type can be instanced from a [User] / [Member] if that fails.
/// Also holds some additional which are relevent to Luro only. These are empty if the type was not instanced from the database.
///
/// Check [LuroUserType] to know how this type was instanced.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Member {
    pub avatar: Option<ImageHash>,
    pub boosting_since: Option<Timestamp>,
    pub communication_disabled_until: Option<Timestamp>,
    /// Only present when fetched from the database
    pub data: Option<MemberData>,
    pub deafened: bool,
    pub flags: MemberFlags,
    pub guild_id: Id<GuildMarker>,
    pub joined_at: Timestamp,
    pub muted: bool,
    pub nickname: Option<String>,
    pub pending: bool,
    pub roles: Vec<Id<RoleMarker>>,
    pub user_id: Id<UserMarker>,
}
