use tracing::{info, warn, error};
use twilight_http::Client;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::guild::LuroGuild;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Attempts to get a user from the cache, otherwise gets the user from the database
    pub async fn get_guild(&self, id: &Id<GuildMarker>, twilight_client: &Client) -> anyhow::Result<LuroGuild> {
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

        let mut response = match data {
            Some(data) => {
                // Flush the new data to disk
                match self.guild_data.write() {
                    Ok(mut guild) => {
                        guild.insert(*id, data.clone());
                    }
                    Err(why) => error!(why = ?why, "guild_data lock is poisoned! Please investigate!")
                }
                data
            }
            None => LuroGuild::new(*id)
        };

        match twilight_client.guild(*id).await {
            Ok(guild) => {response.update_guild(guild.model().await?);},
            Err(why) => info!(why = ?why, "Failed to update user"),
        }

        Ok(response)
    }
}
