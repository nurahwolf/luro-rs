use sqlx::postgres::PgQueryResult;
use twilight_model::{guild::Guild, gateway::payload::incoming::GuildUpdate};

use crate::{sync::GuildSync, LuroDatabase};

impl LuroDatabase {
    pub async fn update_guild(&self, guild: impl Into<GuildSync>) -> anyhow::Result<u64> {
        Ok(match guild.into() {
            GuildSync::Guild(guild) => handle_guild(self, guild).await?.rows_affected(),
            GuildSync::GuildUpdate(guild) => handle_guild_update(self, guild).await?.rows_affected(),
        })
    }
}

async fn handle_guild(db: &LuroDatabase, guild: Guild) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_file!(
        "queries/guilds/update_guild.sql",
        guild.id.get() as i64,
        guild.owner_id.get() as i64,
        guild.name,
    ).execute(&db.pool).await
}

async fn handle_guild_update(db: &LuroDatabase, guild: Box<GuildUpdate>) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query_file!(
        "queries/guilds/update_guild.sql",
        guild.id.get() as i64,
        guild.owner_id.get() as i64,
        guild.name,
    ).execute(&db.pool).await
}
