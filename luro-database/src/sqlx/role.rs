use luro_model::role::LuroRole;
use sqlx::types::Json;
use twilight_model::{
    gateway::payload::incoming::{RoleCreate, RoleDelete, RoleUpdate},
    guild::{Role, RoleTags},
    id::{marker::GuildMarker, Id},
    util::ImageHash,
};

mod count_roles;
mod delete_role;
mod update_role;

pub struct DbRole {
    pub colour: i32,
    pub deleted: bool,
    pub hoist: bool,
    pub icon: Option<Json<ImageHash>>,
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
            icon: role.icon.map(Json),
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

pub enum DbRoleType {
    DbRole(DbRole),
    LuroRole(LuroRole),
    Role(Role, Id<GuildMarker>),
    RoleCreate(RoleCreate),
    RoleDelete(RoleDelete),
    RoleUpdate(RoleUpdate),
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
