use luro_model::guild::LuroGuild;

use crate::{DatabaseGuild, LuroDatabase};

impl LuroDatabase {
    pub async fn handle_luro_guild(&self, guild: LuroGuild) -> Result<DatabaseGuild, sqlx::Error> {
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
            guild.guild_id.get() as i64,
            guild.owner_id.get() as i64,
            guild.name
        )
        .fetch_one(&self.pool)
        .await
    }
}
