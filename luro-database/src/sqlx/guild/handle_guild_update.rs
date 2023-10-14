use twilight_model::gateway::payload::incoming::GuildUpdate;

use crate::{DatabaseGuild, LuroDatabase};

impl LuroDatabase {
    pub async fn handle_guild_update(&self, guild: Box<GuildUpdate>) -> Result<DatabaseGuild, sqlx::Error> {
        sqlx::query_as!(
            DatabaseGuild,
            "INSERT INTO guilds (
                guild_id,
                owner_id,
                name
            ) VALUES
                ($1, $2, $3)
            ON CONFLICT
                (guild_id)
            DO UPDATE SET
                owner_id = $2,
                name = $3
            RETURNING
                guild_id,
                owner_id,
                name",
            guild.id.get() as i64,
            guild.owner_id.get() as i64,
            guild.name
        )
        .fetch_one(&self.pool)
        .await
    }
}
