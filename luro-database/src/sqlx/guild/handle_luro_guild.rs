use crate::{DatabaseGuild, LuroDatabase, LuroGuild};

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
                *",
            guild.guild_id,
            guild.owner_id,
            guild.name
        )
        .fetch_one(&self.pool)
        .await
    }
}
