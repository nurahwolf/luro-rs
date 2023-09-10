use anyhow::anyhow;
use luro_model::{database_driver::LuroDatabaseDriver, guild::LuroGuild};
use tracing::{error, info};
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{LuroDatabase, LuroDatabaseItem};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Fetches a guild in the following priority order
    ///
    /// Luro Cache -> Luro Database -> Twilight Cache -> Twilight Client
    pub async fn get_guild(&self, id: &Id<GuildMarker>) -> anyhow::Result<LuroGuild> {
        // Attempt to fetch the data from Luro's cache
        match self.guild_data.read() {
            Ok(data) => {
                if let Some(data) = data.get(id) {
                    return Ok(data.clone());
                }
            }
            Err(why) => error!(why = ?why, "guild_data lock is poisoned! Please investigate!"),
        };

        info!(id = ?id, "guild is not in Luro's cache, fetching from Luro's Database");
        if let Ok(data) = LuroGuild::get_item(&id.get(), ()).await {
            match self.guild_data.write() {
                Ok(mut db) => {
                    if let Some(data) = db.insert(*id, data.clone()) {
                        return Ok(data.clone());
                    }
                }
                Err(why) => error!(why = ?why, "guild_data lock is poisoned! Please investigate!"),
            };
            return Ok(data);
        }

        info!(id = ?id, "guild is not in Luro's cache, fetching from Twilight's Cache");
        if let Some(data) = self.config.cache.guild(*id) {
            return Ok(LuroGuild::from(data));
        }

        info!(id = ?id, "guild is not in Luro's cache, fetching from Twilight's Client");
        if let Ok(data) = self.config.twilight_client.guild(*id).await {
            return Ok(LuroGuild::from(data.model().await?));
        }

        Err(anyhow!(
            "Could not find any data relating to the guild. Am I still in that guild?"
        ))
    }
}
