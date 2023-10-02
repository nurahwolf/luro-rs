use luro_model::guild::LuroGuild;
use twilight_model::guild::Guild;

use crate::{DatabaseGuild, LuroDatabase};

impl LuroDatabase {
    pub async fn handle_guild(&self, guild: Guild) -> Result<LuroGuild, sqlx::Error> {
        let query = sqlx::query_as!(
            DatabaseGuild,
            "INSERT INTO guilds (guild_id, owner_id) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET owner_id = $2 RETURNING guild_id, owner_id",
            guild.id.get() as i64,
            guild.owner_id.get() as i64
        );

        query.fetch_one(&self.0).await.map(|x| x.luro_guild())
    }
}