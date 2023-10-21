use time::OffsetDateTime;
use twilight_model::{
    guild::Member,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
    util::{image_hash::ImageHashParseError, ImageHash},
};

use crate::DbMember;

/// A warpper around [User], with [Member] details if [Id<GuildMarker>] was present on type creation.
/// Details are primarily fetched from the database, but this type can be instanced from a [User] / [Member] if that fails.
/// Also holds some additional which are relevent to Luro only. These are empty if the type was not instanced from the database.
///
/// Check [LuroUserType] to know how this type was instanced.
#[derive(Clone, Debug)]
pub struct LuroMember {
    pub avatar: Option<ImageHash>,
    pub boosting_since: Option<time::OffsetDateTime>,
    pub communication_disabled_until: Option<time::OffsetDateTime>,
    pub joined_at: time::OffsetDateTime,
    pub deafened: bool,
    pub flags: i64,
    pub guild_id: i64,
    pub muted: bool,
    pub nickname: Option<String>,
    pub pending: bool,
    pub roles: Vec<Id<RoleMarker>>,
    pub user_id: i64,
}

impl TryFrom<DbMember> for LuroMember {
    type Error = ImageHashParseError;

    fn try_from(db_member: DbMember) -> Result<Self, Self::Error> {
        Ok(Self {
            avatar: match db_member.avatar {
                Some(avatar) => Some(ImageHash::parse(avatar.as_bytes())?),
                None => None,
            },
            boosting_since: db_member.boosting_since,
            communication_disabled_until: db_member.communication_disabled_until,
            deafened: db_member.deafened,
            flags: db_member.member_flags,
            guild_id: db_member.guild_id,
            muted: db_member.muted,
            nickname: db_member.nickname,
            pending: db_member.pending,
            user_id: db_member.user_id,
            joined_at: db_member.joined_at.unwrap(),
            roles: db_member.roles.unwrap().into_iter().map(|x| Id::new(x as u64)).collect::<Vec<_>>(),
        })
    }
}
