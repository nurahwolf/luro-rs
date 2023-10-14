use crate::{DatabaseGuild, LuroDatabase};

impl LuroDatabase {
    pub async fn get_all_guilds(&self) -> Result<Vec<DatabaseGuild>, sqlx::Error> {
        sqlx::query_as!(
            DatabaseGuild,
            "
            SELECT *
            FROM guilds
            "
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_guild(&self, id: i64) -> Result<Option<DatabaseGuild>, sqlx::Error> {
        sqlx::query_as!(DatabaseGuild, "SELECT * FROM guilds WHERE guild_id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }
}
