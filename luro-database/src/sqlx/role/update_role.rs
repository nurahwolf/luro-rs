use twilight_model::{
    gateway::payload::incoming::{RoleCreate, RoleUpdate},
    guild::{Role, RoleTags},
    id::{marker::GuildMarker, Id},
};

use sqlx::types::Json;

use crate::{DbRole, DbRoleType, LuroDatabase};

impl LuroDatabase {
    pub async fn update_role(&self, role: impl Into<DbRoleType>) -> Result<DbRole, sqlx::Error> {
        let role: DbRoleType = role.into();

        match role {
            DbRoleType::DbRole(role) => handle_role(self, role).await,
            DbRoleType::LuroRole(role) => handle_role(self, role).await,
            DbRoleType::Role(role, guild_id) => handle_twilight_role(self, role, guild_id).await,
            DbRoleType::RoleCreate(role) => handle_role_create(self, role).await,
            DbRoleType::RoleDelete(role) => self.delete_role(role.role_id.get() as i64).await,
            DbRoleType::RoleUpdate(role) => handle_role_update(self, role).await,
        }
    }
}

async fn handle_role(db: &LuroDatabase, role: impl Into<DbRole>) -> Result<DbRole, sqlx::Error> {
    let role = role.into();
    sqlx::query_as!(
        DbRole,
        "INSERT INTO guild_roles (
            colour,
            role_flags,
            guild_id,
            hoist,
            icon,
            managed,
            mentionable,
            role_name,
            permissions,
            position,
            role_id,
            tags,
            unicode_emoji
        ) VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        ON CONFLICT
            (role_id)
        DO UPDATE SET
            colour = $1,
            role_flags = $2,
            guild_id = $3,
            hoist = $4,
            icon = $5,
            managed = $6,
            mentionable = $7,
            role_name = $8,
            permissions = $9,
            position = $10,
            tags = $12,
            unicode_emoji = $13
        RETURNING
            colour,
            deleted,
            role_flags,
            guild_id,
            hoist,
            icon,
            managed,
            mentionable,
            role_name,
            permissions,
            position,
            role_id,
            tags as \"tags: Json<RoleTags>\",
            unicode_emoji",
        role.colour,
        role.role_flags,
        role.guild_id,
        role.hoist,
        role.icon as _,
        role.managed,
        role.mentionable,
        role.role_name,
        role.permissions,
        role.position,
        role.role_id,
        role.tags as _,
        role.unicode_emoji,
    )
    .fetch_one(&db.pool)
    .await
}

async fn handle_role_create(db: &LuroDatabase, role: RoleCreate) -> Result<DbRole, sqlx::Error> {
    sqlx::query_as!(
        DbRole,
        "INSERT INTO guild_roles (
            colour,
            role_flags,
            guild_id,
            hoist,
            icon,
            managed,
            mentionable,
            role_name,
            permissions,
            position,
            role_id,
            tags,
            unicode_emoji
        ) VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        ON CONFLICT
            (role_id)
        DO UPDATE SET
            colour = $1,
            role_flags = $2,
            guild_id = $3,
            hoist = $4,
            icon = $5,
            managed = $6,
            mentionable = $7,
            role_name = $8,
            permissions = $9,
            position = $10,
            tags = $12,
            unicode_emoji = $13
        RETURNING
            colour,
            deleted,
            role_flags,
            guild_id,
            hoist,
            icon,
            managed,
            mentionable,
            role_name,
            permissions,
            position,
            role_id,
            tags as \"tags: Json<RoleTags>\",
            unicode_emoji",
        role.role.color as i32,
        role.role.flags.bits() as i32,
        role.guild_id.get() as i64,
        role.role.hoist,
        role.role.icon.map(Json) as _,
        role.role.managed,
        role.role.mentionable,
        role.role.name,
        role.role.permissions.bits() as i32,
        role.role.position,
        role.role.id.get() as i64,
        role.role.tags.map(Json) as _,
        role.role.unicode_emoji,
    )
    .fetch_one(&db.pool)
    .await
}

async fn handle_role_update(db: &LuroDatabase, role: RoleUpdate) -> Result<DbRole, sqlx::Error> {
    sqlx::query_as!(
        DbRole,
        "INSERT INTO guild_roles (
            colour,
            role_flags,
            guild_id,
            hoist,
            icon,
            managed,
            mentionable,
            role_name,
            permissions,
            position,
            role_id,
            tags,
            unicode_emoji
        ) VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        ON CONFLICT
            (role_id)
        DO UPDATE SET
            colour = $1,
            role_flags = $2,
            guild_id = $3,
            hoist = $4,
            icon = $5,
            managed = $6,
            mentionable = $7,
            role_name = $8,
            permissions = $9,
            position = $10,
            tags = $12,
            unicode_emoji = $13
        RETURNING
            colour,
            deleted,
            role_flags,
            guild_id,
            hoist,
            icon,
            managed,
            mentionable,
            role_name,
            permissions,
            position,
            role_id,
            tags as \"tags: Json<RoleTags>\",
            unicode_emoji",
        role.role.color as i32,
        role.role.flags.bits() as i32,
        role.guild_id.get() as i64,
        role.role.hoist,
        role.role.icon.map(Json) as _,
        role.role.managed,
        role.role.mentionable,
        role.role.name,
        role.role.permissions.bits() as i32,
        role.role.position,
        role.role.id.get() as i64,
        role.role.tags.map(Json) as _,
        role.role.unicode_emoji,
    )
    .fetch_one(&db.pool)
    .await
}

async fn handle_twilight_role(db: &LuroDatabase, role: Role, guild_id: Id<GuildMarker>) -> Result<DbRole, sqlx::Error> {
    sqlx::query_as!(
        DbRole,
        "INSERT INTO guild_roles (
            colour,
            role_flags,
            guild_id,
            hoist,
            icon,
            managed,
            mentionable,
            role_name,
            permissions,
            position,
            role_id,
            tags,
            unicode_emoji
        ) VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        ON CONFLICT
            (role_id)
        DO UPDATE SET
            colour = $1,
            role_flags = $2,
            guild_id = $3,
            hoist = $4,
            icon = $5,
            managed = $6,
            mentionable = $7,
            role_name = $8,
            permissions = $9,
            position = $10,
            tags = $12,
            unicode_emoji = $13
        RETURNING
            colour,
            deleted,
            role_flags,
            guild_id,
            hoist,
            icon,
            managed,
            mentionable,
            role_name,
            permissions,
            position,
            role_id,
            tags as \"tags: Json<RoleTags>\",
            unicode_emoji",
        role.color as i32,
        role.flags.bits() as i32,
        guild_id.get() as i64,
        role.hoist,
        role.icon.map(Json) as _,
        role.managed,
        role.mentionable,
        role.name,
        role.permissions.bits() as i32,
        role.position,
        role.id.get() as i64,
        role.tags.map(Json) as _,
        role.unicode_emoji,
    )
    .fetch_one(&db.pool)
    .await
}
