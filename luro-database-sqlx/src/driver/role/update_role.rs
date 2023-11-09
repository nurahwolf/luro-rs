use luro_model::{sync::RoleSync, types::Role};
use sqlx::types::Json;
use twilight_model::{
    gateway::payload::incoming::{RoleCreate, RoleUpdate},
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};

use crate::SQLxDriver;

impl SQLxDriver {
    pub async fn update_role(&self, role: impl Into<RoleSync<'_>>) -> Result<u64, sqlx::Error> {
        match role.into() {
            RoleSync::Role(role) => handle_luro_role(self, role).await,
            RoleSync::TwilightRole(guild_id, role) => handle_twilight_role(self, role, guild_id).await,
            RoleSync::RoleCreate(role) => handle_role_create(self, role).await,
            RoleSync::RoleDelete(role) => self.role_delete(role.role_id.get() as i64).await,
            RoleSync::RoleUpdate(role) => handle_role_update(self, role).await,
            RoleSync::RoleId(guild_id, role_id) => handle_role_id(self, guild_id, role_id).await,
        }
    }
}

async fn handle_role_id(db: &SQLxDriver, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/guild_roles/update_twilight_role_id.sql",
        guild_id.get() as i64,
        role_id.get() as i64,
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_luro_role(db: &SQLxDriver, role: &Role) -> Result<u64, sqlx::Error> {
    if let Some(data) = &role.data {
        db.update_role_data(data, role.guild_id, role.role_id).await?;
    }

    sqlx::query_file!(
        "queries/guild_roles/update_role.sql",
        role.colour as i32,
        role.guild_id.get() as i64,
        role.hoist,
        role.icon.map(Json) as _,
        role.managed,
        role.mentionable,
        role.permissions.bits() as i32,
        role.position,
        role.flags.bits() as i32,
        role.role_id.get() as i64,
        role.name,
        role.tags.as_ref().map(Json) as _,
        role.unicode_emoji,
    )
    .execute(&db.pool)
    .await
    .map(|x| x.rows_affected())
}

async fn handle_role_create(db: &SQLxDriver, role: &RoleCreate) -> Result<u64, sqlx::Error> {
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
        role.role.icon.map(Json) as _,
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

async fn handle_role_update(db: &SQLxDriver, role: &RoleUpdate) -> Result<u64, sqlx::Error> {
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
        role.role.icon.map(Json) as _,
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

async fn handle_twilight_role(db: &SQLxDriver, role: &twilight_model::guild::Role, guild_id: Id<GuildMarker>) -> Result<u64, sqlx::Error> {
    sqlx::query_file!(
        "queries/guild_roles/update_role.sql",
        role.color as i32,
        guild_id.get() as i64,
        role.hoist,
        role.icon.map(Json) as _,
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
