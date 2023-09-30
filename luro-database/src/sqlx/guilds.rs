use std::collections::HashMap;

use futures_util::TryStreamExt;
use luro_model::guild::LuroGuild;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{LuroDatabase, DatabaseGuild};

impl From<LuroGuild> for DatabaseGuild {
    fn from(guild: LuroGuild) -> Self {
        Self {
            guild_id: guild.guild_id,
        }
    }
}

impl LuroDatabase {
    pub async fn get_all_guilds(&self) -> HashMap<Id<GuildMarker>, DatabaseGuild> {
        let mut guilds = HashMap::new();
        let query = sqlx::query_as!(
            DatabaseGuild,
            "
            SELECT *
            FROM guilds
            "
        );

        for guild in (query.fetch(&self.0).try_next().await).into_iter().flatten() {
            guilds.insert(Id::new(guild.guild_id as u64), guild);
        }

        guilds
    }

    pub async fn update_guild(&self, guild: impl Into<LuroGuild>) -> Result<DatabaseGuild, sqlx::Error> {
        let query = sqlx::query_as!(
            DatabaseGuild,
            "INSERT INTO guilds (guild_id) VALUES ($1) ON CONFLICT (guild_id) DO UPDATE SET guild_id = $1 RETURNING guild_id",
            guild.into().guild_id
        );

        query.fetch_one(&self.0).await
    }

    pub async fn get_guild(&self, id: i64) -> Result<Option<DatabaseGuild>, sqlx::Error> {
        let query = sqlx::query_as!(DatabaseGuild, "SELECT * FROM guilds WHERE guild_id = $1", id);

        let data = query.fetch_optional(&self.0).await?;

        Ok(data)
    }
}
