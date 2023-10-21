use sqlx::{types::Json, FromRow};

use twilight_model::{
    gateway::payload::incoming::{RoleCreate, RoleDelete, RoleUpdate},
    guild::{Permissions, Role, RoleFlags, RoleTags},
    id::{marker::GuildMarker, Id},
    util::ImageHash,
};

use crate::LuroRole;

mod count_roles;
mod delete_role;
mod get_role;
mod get_roles;
mod update_role;

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
    pub name: String,
    pub permissions: i64,
    pub position: i64,
    pub flags: i64,
    pub tags: Option<Json<RoleTags>>,
    pub unicode_emoji: Option<String>,
}

impl From<LuroRole> for DbRole {
    fn from(role: LuroRole) -> Self {
        Self {
            colour: role.colour as i32,
            deleted: role.deleted,
            flags: role.flags.bits() as i64,
            guild_id: role.guild_id.get() as i64,
            hoist: role.hoist,
            icon: role.icon.map(|x| x.to_string()),
            managed: role.managed,
            mentionable: role.mentionable,
            name: role.name,
            permissions: role.permissions.bits() as i64,
            position: role.position,
            role_id: role.id.get() as i64,
            tags: role.tags.map(Json),
            unicode_emoji: role.unicode_emoji,
        }
    }
}

impl From<DbRole> for LuroRole {
    fn from(role: DbRole) -> Self {
        Self {
            colour: role.colour as u32,
            deleted: role.deleted,
            flags: RoleFlags::from_bits_retain(role.flags as u64),
            guild_id: Id::new(role.guild_id as u64),
            hoist: role.hoist,
            icon: role.icon.map(|x| ImageHash::parse(x.as_bytes()).unwrap()), // TODO: Error handling
            managed: role.managed,
            mentionable: role.mentionable,
            name: role.name,
            permissions: Permissions::from_bits_retain(role.permissions as u64),
            position: role.position,
            id: Id::new(role.role_id as u64),
            tags: role.tags.map(|x| x.0),
            unicode_emoji: role.unicode_emoji,
        }
    }
}

impl From<DbRole> for Role {
    fn from(role: DbRole) -> Self {
        Self {
            color: role.colour as u32,
            flags: RoleFlags::from_bits_retain(role.flags as u64),
            hoist: role.hoist,
            icon: role.icon.map(|x| ImageHash::parse(x.as_bytes()).unwrap()), // TODO: Error handling
            managed: role.managed,
            mentionable: role.mentionable,
            name: role.name,
            permissions: Permissions::from_bits_retain(role.permissions as u64),
            position: role.position,
            id: Id::new(role.role_id as u64),
            tags: role.tags.map(|x| x.0),
            unicode_emoji: role.unicode_emoji,
        }
    }
}

pub enum DbRoleType {
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

// impl From<DbRoleType> for DbRole {
//     fn from(role: DbRoleType) -> Self {
//         match role {
//             DbRoleType::Role(role, guild_id) => Self {
//                 colour: role.color as i32,
//                 deleted: false,
//                 flags: role.flags.bits() as i64,
//                 hoist: role.hoist,
//                 icon: role.icon.map(Json),
//                 guild_id: guild_id.get() as i64,
//                 managed: role.managed,
//                 mentionable: role.mentionable,
//                 name: role.name,
//                 permissions: role.permissions.bits() as i64,
//                 position: role.position,
//                 role_id: role.id.get() as i64,
//                 tags: role.tags.map(Json),
//                 unicode_emoji: role.unicode_emoji,
//             },
//         }
//     }
// }
