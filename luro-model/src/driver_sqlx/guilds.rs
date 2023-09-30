use anyhow::Result;
use std::collections::HashMap;

use twilight_model::id::{marker::GuildMarker, Id};

use crate::guild::LuroGuild;

use super::PostgresDriver;

#[derive(Clone)]
pub struct DatabaseGuild {
    pub guild_id: i64,
}

impl From<LuroGuild> for DatabaseGuild {
    fn from(guild: LuroGuild) -> Self {
        Self {
            guild_id: guild.guild_id,
        }
    }
}

// impl LuroDatabaseItem for DatabaseGuild {
//     type Item = DatabaseGuild;
//     type Id = i64;
//     type Container = BTreeMap<Self::Id, Self::Item>;
//     type Driver = PostgresDriver;

//     async fn add_item(driver: Self::Driver, item: &Self::Item) -> anyhow::Result<()> {
//         let query = sqlx::query_as!(
//             DatabaseGuild,
//             "INSERT INTO guilds (guild_id) VALUES ($1) ON CONFLICT (guild_id) DO UPDATE SET guild_id = $1",
//             item.guild_id
//         );
//         query.execute(&driver.0).await?;

//         Ok(())
//     }

//     async fn add_items(driver: Self::Driver, items: &Self::Container) -> anyhow::Result<()> {
//         for item in items.values() {
//             let query = sqlx::query_as!(
//                 DatabaseGuild,
//                 "INSERT INTO guilds (guild_id) VALUES ($1) ON CONFLICT (guild_id) DO UPDATE SET guild_id = $1",
//                 item.guild_id
//             );
//             query.execute(&driver.0).await?;
//         }
//         Ok(())
//     }

//     async fn get_item(id: &Self::Id, driver: Self::Driver) -> anyhow::Result<Option<Self::Item>> {
//         let query = sqlx::query_as!(Self::Item, "SELECT * FROM guilds WHERE guild_id = $1", id);

//         let data = query
//             .fetch_optional(&driver.0)
//             .await?;

//         Ok(data)
//     }

//     async fn get_items(ids: Vec<&Self::Id>, driver: Self::Driver) -> anyhow::Result<Self::Container> {
//         let mut items = BTreeMap::new();
//         for id in ids {
//             let query = sqlx::query_as!(Self::Item, "SELECT * FROM guilds WHERE guild_id = $1", id);
//             if let Some(data) = query.fetch_optional(&driver.0).await? {
//                 items.insert(data.guild_id, data);
//             }
//         }
//         Ok(items)
//     }

//     async fn modify_item(driver: Self::Driver, _id: &Self::Id, item: &Self::Item) -> anyhow::Result<Option<Self::Item>> {
//         let query = sqlx::query_as!(
//             DatabaseGuild,
//             "INSERT INTO guilds (guild_id) VALUES ($1) ON CONFLICT (guild_id) DO UPDATE SET guild_id = $1 RETURNING guild_id",
//             item.guild_id
//         );
//         Ok(query.fetch_optional(&driver.0).await?)
//     }

//     async fn modify_items(driver: Self::Driver, items: &Self::Container) -> anyhow::Result<Self::Container> {
//         let mut old_items = BTreeMap::new();
//         for (id, item) in items {
//             let query = sqlx::query_as!(
//                 DatabaseGuild,
//                 "INSERT INTO guilds (guild_id) VALUES ($1) ON CONFLICT (guild_id) DO UPDATE SET guild_id = $1 RETURNING guild_id",
//                 item.guild_id
//             );

//             if let Some(data) = query.fetch_optional(&driver.0).await? {
//                 old_items.insert(data.guild_id, data);
//             }
//         }
//         Ok(old_items)
//     }

//     async fn remove_item(id: &Self::Id, driver: Self::Driver) -> anyhow::Result<Option<Self::Item>> {
//         let query = sqlx::query_as!(Self::Item, "SELECT * FROM guilds WHERE guild_id = $1", id);

//         let data = query
//             .fetch_optional(&driver.0)
//             .await?;

//         Ok(data)
//     }

//     async fn remove_items(ids: Vec<&Self::Id>, driver: Self::Driver) -> anyhow::Result<Self::Container> {
//         let mut old_items = BTreeMap::new();
//         for id in ids {
//             let query = sqlx::query_as!(Self::Item, "SELECT * FROM guilds WHERE guild_id = $1", id);
//             if let Some(data) = query.fetch_optional(&driver.0).await? {
//                 old_items.insert(data.guild_id, data);
//             }
//         }
//         Ok(old_items)
//     }
// }

impl PostgresDriver {
    pub async fn get_all_guilds(&self) -> Result<HashMap<Id<GuildMarker>, DatabaseGuild>> {
        let mut guilds = HashMap::new();
        for guild in sqlx::query_as!(
            DatabaseGuild,
            "
            SELECT *
            FROM guilds
            "
        )
        .fetch_all(&self.0) // -> Vec<Country>
        .await?
        {
            guilds.insert(Id::new(guild.guild_id as u64), guild);
        }

        Ok(guilds)
    }

    pub async fn update_guild(&self, guild: impl Into<LuroGuild>) -> Result<DatabaseGuild, sqlx::Error> {
        let query = sqlx::query_as!(
            DatabaseGuild,
            "INSERT INTO guilds (guild_id) VALUES ($1) ON CONFLICT (guild_id) DO UPDATE SET guild_id = $1 RETURNING guild_id",
            guild.into().guild_id
        );
        
        query.fetch_one(&self.0).await
    }

    pub async fn get_guild(&self, id: i64) -> anyhow::Result<Option<DatabaseGuild>> {
        let query = sqlx::query_as!(DatabaseGuild, "SELECT * FROM guilds WHERE guild_id = $1", id);

        let data = query
            .fetch_optional(&self.0)
            .await?;

        Ok(data)
    }
}
