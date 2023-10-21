use sqlx::types::Json;
use time::OffsetDateTime;
use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate},
    guild::{Member, PartialMember},
    id::{marker::GuildMarker, Id},
    util::ImageHash,
};

use crate::LuroMember;

mod get_member;
mod get_members;
mod update_member;

pub enum DbMemberType {
    Member(Id<GuildMarker>, Member),
    MemberAdd(Box<MemberAdd>),
    MemberChunk(MemberChunk),
    MemberRemove(MemberRemove),
    MemberUpdate(Box<MemberUpdate>),
    PartialMember(Id<GuildMarker>, PartialMember),
    LuroMember(LuroMember),
}

#[derive(Clone, Debug)]
pub struct DbMember {
    pub user_id: i64,
    pub guild_id: i64,
    pub avatar: Option<Json<ImageHash>>,
    pub boosting_since: Option<time::OffsetDateTime>,
    pub communication_disabled_until: Option<time::OffsetDateTime>,
    pub deafened: bool,
    pub flags: i32,
    pub muted: bool,
    pub nickname: Option<String>,
    pub pending: bool,
    // pub roles: Vec<i64>,
}

impl DbMember {
    pub fn new(guild_id: i64, member: Member) -> Self {
        Self {
            user_id: member.user.id.get() as i64,
            guild_id,
            avatar: member.avatar.map(Json) as _,
            pending: member.pending,
            boosting_since: member
                .premium_since
                .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
            communication_disabled_until: member
                .communication_disabled_until
                .map(|x| OffsetDateTime::from_unix_timestamp(x.as_secs()).unwrap()),
            deafened: member.deaf,
            flags: member.flags.bits() as i32,
            muted: member.mute,
            nickname: member.nick,
        }
    }
}
