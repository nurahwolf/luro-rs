
mod joined_at;
mod boosing_since;
mod communication_disabled_until;

use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use twilight_model::{
    id::{
        marker::RoleMarker,
        Id,
    },
    util::{image_hash::ImageHashParseError, ImageHash},
};

use crate::{DbMember, LuroRole};

/// A warpper around [User], with [Member] details if [Id<GuildMarker>] was present on type creation.
/// Details are primarily fetched from the database, but this type can be instanced from a [User] / [Member] if that fails.
/// Also holds some additional which are relevent to Luro only. These are empty if the type was not instanced from the database.
///
/// Check [LuroUserType] to know how this type was instanced.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LuroMember {
    pub avatar: Option<String>,
    #[serde(skip)]
    pub boosting_since: Option<time::OffsetDateTime>,
    #[serde(skip)]
    pub communication_disabled_until: Option<time::OffsetDateTime>,
    #[serde(skip, default = "default")]
    pub joined_at: time::OffsetDateTime,
    pub deafened: bool,
    pub flags: i64,
    pub guild_id: i64,
    pub muted: bool,
    pub nickname: Option<String>,
    pub pending: bool,
    pub roles: HashMap<Id<RoleMarker>, LuroRole>,
    pub user_id: i64,
}

fn default() -> time::OffsetDateTime {
    time::OffsetDateTime::now_utc()
}
