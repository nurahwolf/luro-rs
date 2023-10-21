use sqlx::types::Json;
use time::OffsetDateTime;
use twilight_model::{
    guild::Member,
    id::{marker::GuildMarker, Id},
    util::ImageHash,
};

use crate::DbMember;

/// A warpper around [User], with [Member] details if [Id<GuildMarker>] was present on type creation.
/// Details are primarily fetched from the database, but this type can be instanced from a [User] / [Member] if that fails.
/// Also holds some additional which are relevent to Luro only. These are empty if the type was not instanced from the database.
///
/// Check [LuroUserType] to know how this type was instanced.
#[derive(Clone, Debug)]
pub struct LuroMember {
    pub avatar: Option<Json<ImageHash>>,
    pub boosting_since: Option<time::OffsetDateTime>,
    pub communication_disabled_until: Option<time::OffsetDateTime>,
    pub deafened: bool,
    pub flags: i32,
    pub guild_id: i64,
    pub muted: bool,
    pub nickname: Option<String>,
    pub pending: bool,
    pub user_id: i64,
}

impl From<DbMember> for LuroMember {
    fn from(db_member: DbMember) -> Self {
        Self {
            avatar: db_member.avatar,
            boosting_since: db_member.boosting_since,
            communication_disabled_until: db_member.communication_disabled_until,
            deafened: db_member.deafened,
            flags: db_member.flags,
            guild_id: db_member.guild_id,
            muted: db_member.muted,
            nickname: db_member.nickname,
            pending: db_member.pending,
            user_id: db_member.user_id,
        }
    }
}

impl From<(Id<GuildMarker>, Member)> for LuroMember {
    fn from(data: (Id<GuildMarker>, Member)) -> Self {
        let (guild_id, member) = data;
        Self {
            avatar: member.avatar.map(Json),
            boosting_since: member
                .premium_since
                .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
            communication_disabled_until: member
                .communication_disabled_until
                .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
            deafened: member.deaf,
            flags: member.flags.bits() as i32,
            guild_id: guild_id.get() as i64,
            muted: member.mute,
            nickname: member.nick,
            pending: member.pending,
            user_id: member.user.id.get() as i64,
        }
    }
}
