use sqlx::FromRow;
use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate},
    guild::{Member, PartialMember},
    id::{marker::GuildMarker, Id},
};

use crate::LuroUserPermissions;

mod get_member;
mod get_members;
mod update_member;
mod clear_roles;

pub enum DbMemberType {
    Member(Id<GuildMarker>, Member),
    MemberAdd(Box<MemberAdd>),
    MemberChunk(MemberChunk),
    MemberRemove(MemberRemove),
    MemberUpdate(Box<MemberUpdate>),
    PartialMember(Id<GuildMarker>, PartialMember),
    // LuroMember(LuroMember)
}

#[derive(Clone, Debug, FromRow)]
pub struct DbMember {
    pub accent_colour: Option<i32>,
    pub avatar_decoration: Option<String>,
    pub user_avatar: Option<String>,
    pub banner: Option<String>,
    pub boosting_since: Option<time::OffsetDateTime>,
    pub bot: bool,
    pub characters: Option<Vec<i32>>,
    pub communication_disabled_until: Option<time::OffsetDateTime>,
    pub deafened: bool,
    pub discriminator: i16,
    pub email: Option<String>,
    pub global_name: Option<String>,
    pub guild_avatar: Option<String>,
    pub guild_id: i64,
    pub joined_at: Option<time::OffsetDateTime>,
    pub locale: Option<String>,
    pub member_flags: i64,
    pub message_edits: Option<i64>,
    pub messages: Option<Vec<i64>>,
    pub mfa_enabled: Option<bool>,
    pub muted: bool,
    pub name: String,
    pub nickname: Option<String>,
    pub pending: bool,
    pub premium_type: Option<i16>,
    pub public_flags: Option<i64>,
    pub roles: Option<Vec<i64>>,
    pub system: Option<bool>,
    pub user_flags: Option<i64>,
    pub user_id: i64,
    pub user_permissions: LuroUserPermissions,
    pub verified: Option<bool>,
    pub warnings: Option<Vec<i64>>,
    pub words_average: Option<i64>,
    pub words_count: Option<i64>,
}
