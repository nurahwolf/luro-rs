use twilight_model::{
    gateway::payload::incoming::{RoleCreate, RoleUpdate},
    guild::Role,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};

use sqlx::{postgres::PgQueryResult, types::Json};

use crate::{DbRoleType, LuroDatabase, LuroRole};

impl LuroDatabase {
    pub async fn update_role(&self, role: impl Into<DbRoleType>) -> Result<PgQueryResult, sqlx::Error> {
        match role.into() {
            DbRoleType::DbRole(_) => todo!(),
            DbRoleType::LuroRole(role) => handle_luro_role(self, role).await,
            DbRoleType::Role(role, guild_id) => handle_twilight_role(self, role, guild_id).await,
            DbRoleType::RoleCreate(role) => handle_role_create(self, role).await,
            DbRoleType::RoleDelete(role) => self.delete_role(role.role_id.get() as i64).await,
            DbRoleType::RoleUpdate(role) => handle_role_update(self, role).await,
            DbRoleType::RoleId(guild_id, role_id) => handle_role_id(self, guild_id, role_id).await,
        }
    }
}

async fn handle_role_id(db: &LuroDatabase, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_file!(
        "queries/guild_roles/update_twilight_role_id.sql",
        guild_id.get() as i64,
        role_id.get() as i64,
    )
    .execute(&db.pool)
    .await
}

async fn handle_luro_role(db: &LuroDatabase, role: LuroRole) -> Result<PgQueryResult, sqlx::Error> {
    if let Some(data) = role.data {
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
        role.tags.map(Json) as _,
        role.unicode_emoji,
    )
    .execute(&db.pool)
    .await
}

async fn handle_role_create(db: &LuroDatabase, role: RoleCreate) -> Result<PgQueryResult, sqlx::Error> {
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
        role.role.tags.map(Json) as _,
        role.role.unicode_emoji,
    )
    .execute(&db.pool)
    .await
}

async fn handle_role_update(db: &LuroDatabase, role: RoleUpdate) -> Result<PgQueryResult, sqlx::Error> {
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
        role.role.tags.map(Json) as _,
        role.role.unicode_emoji,
    )
    .execute(&db.pool)
    .await
}

async fn handle_twilight_role(db: &LuroDatabase, role: Role, guild_id: Id<GuildMarker>) -> Result<PgQueryResult, sqlx::Error> {
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
        role.tags.map(Json) as _,
        role.unicode_emoji,
    )
    .execute(&db.pool)
    .await
}
