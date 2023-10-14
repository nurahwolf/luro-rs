use sqlx::types::Json;
use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate},
    guild::Member,
    id::{marker::GuildMarker, Id},
    util::ImageHash,
};

mod get_member;
mod get_members;
mod update_member;

pub enum DbMemberType {
    Member(Member, Id<GuildMarker>),
    MemberAdd(Box<MemberAdd>),
    MemberChunk(MemberChunk),
    MemberRemove(MemberRemove),
    MemberUpdate(Box<MemberUpdate>),
}

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
