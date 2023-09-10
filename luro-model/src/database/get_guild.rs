use tracing::{error, info, warn};
use twilight_model::id::{marker::GuildMarker, Id};

use crate::guild::LuroGuild;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Attempts to get a user from the cache, otherwise gets the user from the database
    pub async fn get_guild(&self, id: &Id<GuildMarker>) -> anyhow::Result<LuroGuild> {
        let mut data = match self.guild_data.read() {
            Ok(data) => data.get(id).cloned(),
            Err(why) => {
                error!(why = ?why, "guild_data lock is poisoned! Please investigate!");
                None
            }
        };

        if data.is_none() {
            info!(id = ?id, "guild is not in the cache, fetching from disk");
            data = match self.driver.get_guild(id.get()).await {
                Ok(data) => Some(data),
                Err(why) => {
                    warn!(why = ?why, "Failed to get guild from the database. Falling back to twilight");
                    None
                }
            }
        }

        let mut data = match data {
            Some(data) => data,
            None => LuroGuild::new(*id),
        };

        match self.twilight_client.guild(*id).await {
            Ok(guild) => {
                data.update_guild(guild.model().await?);
            }
            Err(why) => info!(why = ?why, "Failed to update guild"),
        }

        match self.guild_data.write() {
            Ok(mut guild) => {
                guild.insert(*id, data.clone());
            }
            Err(why) => error!(why = ?why, "guild_data lock is poisoned! Please investigate!"),
        }

        Ok(data)
    }
}
