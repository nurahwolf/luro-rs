use std::collections::HashMap;

use futures_util::TryStreamExt;
use luro_model::guild::LuroGuild;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{DatabaseGuild, LuroDatabase};

impl DatabaseGuild {
    pub fn luro_guild(&self) -> LuroGuild {
        LuroGuild::new(Id::new(self.guild_id as u64), Id::new(self.owner_id as u64))
    }
}

impl LuroDatabase {
    pub async fn get_all_guilds(&self) -> HashMap<Id<GuildMarker>, LuroGuild> {
        let mut guilds = HashMap::new();
        let query = sqlx::query_as!(
            DatabaseGuild,
            "
            SELECT *
            FROM guilds
            "
        );

        for guild in (query.fetch(&self.0).try_next().await).into_iter().flatten() {
            guilds.insert(Id::new(guild.guild_id as u64), guild.luro_guild());
        }

        guilds
    }

    pub async fn update_guild(&self, guild: impl Into<LuroGuild>) -> Result<LuroGuild, sqlx::Error> {
        let guild = guild.into();
        let query = sqlx::query_as!(
            DatabaseGuild,
            "INSERT INTO guilds (guild_id, owner_id) VALUES ($1, $2) ON CONFLICT (guild_id) DO UPDATE SET owner_id = $2 RETURNING guild_id, owner_id",
            guild.guild_id.get() as i64,
            guild.owner_id.get() as i64
        );

        query.fetch_one(&self.0).await.map(|x|x.luro_guild())
    }

    pub async fn get_guild(&self, id: i64) -> Result<Option<LuroGuild>, sqlx::Error> {
        let query = sqlx::query_as!(DatabaseGuild, "SELECT * FROM guilds WHERE guild_id = $1", id);
        
        query.fetch_optional(&self.0).await.map(|x: Option<DatabaseGuild>|x.map(|x|x.luro_guild()))
    }
}
