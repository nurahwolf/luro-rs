use sqlx::types::Json;
use twilight_model::{
    gateway::payload::incoming::{RoleCreate, RoleDelete, RoleUpdate},
    guild::Role,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};

use crate::database::sqlx::{Database, Error};

impl Database {
    pub async fn update_role(&self, role: impl Into<RoleSync<'_>>) -> Result<u64, Error> {
        match role.into() {
            RoleSync::TwilightRole(guild_id, role) => Ok(handle_twilight_role(self, role, guild_id).await?),
            RoleSync::RoleCreate(role) => Ok(handle_role_create(self, role).await?),
            RoleSync::RoleDelete(role) => Ok(role_delete(self, role).await?),
            RoleSync::RoleUpdate(role) => Ok(handle_role_update(self, role).await?),
            RoleSync::RoleId(guild_id, role_id) => Ok(handle_role_id(self, guild_id, role_id).await?),
        }
    }
}

async fn role_delete(_db: &Database, _role: &RoleDelete) -> Result<u64, sqlx::Error> {
    Ok(0)
}

async fn handle_role_id(db: &Database, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/guild_roles/update_twilight_role_id.sql",
        guild_id.get() as i64,
        role_id.get() as i64,
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_role_create(db: &Database, role: &RoleCreate) -> Result<u64, sqlx::Error> {
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
            (role_id, guild_id)
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
            ",
        role.role.color as i32,
        role.role.flags.bits() as i32,
        role.guild_id.get() as i64,
        role.role.hoist,
        role.role.icon.map(|x| x.to_string()),
        role.role.managed,
        role.role.mentionable,
        role.role.name,
        role.role.permissions.bits() as i32,
        role.role.position,
        role.role.id.get() as i64,
        role.role.tags.as_ref().map(Json) as _,
        role.role.unicode_emoji,
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_role_update(db: &Database, role: &RoleUpdate) -> Result<u64, sqlx::Error> {
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
            (role_id, guild_id)
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
            unicode_emoji = $13",
        role.role.color as i32,
        role.role.flags.bits() as i32,
        role.guild_id.get() as i64,
        role.role.hoist,
        role.role.icon.map(|x| x.to_string()),
        role.role.managed,
        role.role.mentionable,
        role.role.name,
        role.role.permissions.bits() as i32,
        role.role.position,
        role.role.id.get() as i64,
        role.role.tags.as_ref().map(Json) as _,
        role.role.unicode_emoji,
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_twilight_role(db: &Database, role: &Role, guild_id: Id<GuildMarker>) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/guild_roles/update_role.sql",
        role.color as i32,
        guild_id.get() as i64,
        role.hoist,
        role.icon.map(|x| x.to_string()),
        role.managed,
        role.mentionable,
        role.permissions.bits() as i32,
        role.position,
        role.flags.bits() as i32,
        role.id.get() as i64,
        role.name,
        role.tags.as_ref().map(Json) as _,
        role.unicode_emoji,
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

pub enum RoleSync<'a> {
    // Role(&'a Role),
    RoleCreate(&'a RoleCreate),
    RoleDelete(&'a RoleDelete),
    RoleId(Id<GuildMarker>, Id<RoleMarker>),
    RoleUpdate(&'a RoleUpdate),
    TwilightRole(Id<GuildMarker>, &'a twilight_model::guild::Role),
}

impl<'a> From<(Id<GuildMarker>, Id<RoleMarker>)> for RoleSync<'a> {
    fn from((guild_id, role_id): (Id<GuildMarker>, Id<RoleMarker>)) -> Self {
        Self::RoleId(guild_id, role_id)
    }
}

// impl<'a> From<&'a Role> for RoleSync<'a> {
//     fn from(role: &'a Role) -> Self {
//         Self::Role(role)
//     }
// }

impl<'a> From<&'a RoleCreate> for RoleSync<'a> {
    fn from(role: &'a RoleCreate) -> Self {
        Self::RoleCreate(role)
    }
}

impl<'a> From<&'a RoleUpdate> for RoleSync<'a> {
    fn from(role: &'a RoleUpdate) -> Self {
        Self::RoleUpdate(role)
    }
}

impl<'a> From<&'a RoleDelete> for RoleSync<'a> {
    fn from(role: &'a RoleDelete) -> Self {
        Self::RoleDelete(role)
    }
}

impl<'a> From<(Id<GuildMarker>, &'a twilight_model::guild::Role)> for RoleSync<'a> {
    fn from((role_id, role): (Id<GuildMarker>, &'a twilight_model::guild::Role)) -> Self {
        Self::TwilightRole(role_id, role)
    }
}
