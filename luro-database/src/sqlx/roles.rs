use luro_model::role::LuroRole;
use sqlx::types::Json;
use twilight_model::{
    guild::{Permissions, Role, RoleFlags},
    id::Id,
};

use crate::DatabaseRole;

mod count_roles;
mod delete_role;
mod update_role;

impl From<Role> for DatabaseRole {
    fn from(role: Role) -> Self {
        Self {
            colour: role.color as i32,
            deleted: false,
            flags: role.flags.bits() as i64,
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

impl From<DatabaseRole> for Role {
    fn from(role: DatabaseRole) -> Self {
        Self {
            color: role.colour as u32,
            hoist: role.hoist,
            icon: role.icon.map(|x| x.0),
            id: Id::new(role.role_id as u64),
            managed: role.managed,
            mentionable: role.mentionable,
            name: role.name,
            permissions: Permissions::from_bits_retain(role.permissions as u64),
            position: role.position,
            flags: RoleFlags::from_bits_retain(role.flags as u64),
            tags: role.tags.map(|x| x.0),
            unicode_emoji: role.unicode_emoji,
        }
    }
}

impl From<DatabaseRole> for LuroRole {
    fn from(role: DatabaseRole) -> Self {
        Self {
            colour: role.colour as u32,
            deleted: role.deleted,
            hoist: role.hoist,
            icon: role.icon.map(|x| x.0),
            id: Id::new(role.role_id as u64),
            managed: role.managed,
            mentionable: role.mentionable,
            name: role.name,
            permissions: Permissions::from_bits_retain(role.permissions as u64),
            position: role.position,
            flags: RoleFlags::from_bits_retain(role.flags as u64),
            tags: role.tags.map(|x| x.0),
            unicode_emoji: role.unicode_emoji,
        }
    }
}
