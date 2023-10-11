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

mod update_guild;
mod handle_guild_update;
mod handle_luro_guild;
mod handle_guild;
mod count_guilds;

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

    pub async fn get_guild(&self, id: i64) -> Result<Option<LuroGuild>, sqlx::Error> {
        let query = sqlx::query_as!(DatabaseGuild, "SELECT * FROM guilds WHERE guild_id = $1", id);

        query
            .fetch_optional(&self.0)
            .await
            .map(|x: Option<DatabaseGuild>| x.map(|x| x.luro_guild()))
    }
}
