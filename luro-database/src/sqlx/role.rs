use sqlx::{types::Json, FromRow};

use twilight_model::{
    gateway::payload::incoming::{RoleCreate, RoleDelete, RoleUpdate},
    guild::{Role, RoleTags},
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};

use crate::LuroRole;

mod count_roles;
mod delete_role;
mod get_role;
mod get_roles;
mod update_role;
mod update_role_data;

#[derive(Clone, Debug, FromRow)]
pub struct DbRole {
    pub colour: i32,
    pub deleted: bool,
    pub hoist: bool,
    pub icon: Option<String>,
    pub role_id: i64,
    pub guild_id: i64,
    pub managed: bool,
    pub mentionable: bool,
    pub role_name: String,
    pub permissions: i64,
    pub position: i64,
    pub role_flags: i64,
    pub tags: Option<Json<RoleTags>>,
    pub unicode_emoji: Option<String>,
}

pub enum DbRoleType {
    RoleId(Id<GuildMarker>, Id<RoleMarker>),
    DbRole(DbRole),
    LuroRole(LuroRole),
    Role(Role, Id<GuildMarker>),
    RoleCreate(RoleCreate),
    RoleDelete(RoleDelete),
    RoleUpdate(RoleUpdate),
}

impl From<(Id<GuildMarker>, Role)> for DbRoleType {
    fn from(role: (Id<GuildMarker>, Role)) -> Self {
        Self::Role(role.1, role.0)
    }
}

impl From<(Id<GuildMarker>, Id<RoleMarker>)> for DbRoleType {
    fn from((guild_id, role_id): (Id<GuildMarker>, Id<RoleMarker>)) -> Self {
        Self::RoleId(guild_id, role_id)
    }
}

impl From<LuroRole> for DbRoleType {
    fn from(role: LuroRole) -> Self {
        Self::LuroRole(role)
    }
}

impl From<DbRole> for DbRoleType {
    fn from(role: DbRole) -> Self {
        Self::DbRole(role)
    }
}

impl From<RoleCreate> for DbRoleType {
    fn from(role: RoleCreate) -> Self {
        Self::RoleCreate(role)
    }
}

impl From<RoleUpdate> for DbRoleType {
    fn from(role: RoleUpdate) -> Self {
        Self::RoleUpdate(role)
    }
}

impl From<RoleDelete> for DbRoleType {
    fn from(role: RoleDelete) -> Self {
        Self::RoleDelete(role)
    }
}
