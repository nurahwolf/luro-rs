use tracing::{info, warn};
use twilight_model::id::{marker::GuildMarker, Id};

use crate::guild::LuroGuild;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Attempts to get a user from the cache, otherwise gets the user from the database
    pub async fn get_guild(&self, id: &Id<GuildMarker>) -> anyhow::Result<LuroGuild> {
        let data = match self.guild_data.read() {
            Ok(data) => data.get(id).cloned(),
            Err(why) => {
                warn!(why = ?why, "guild_data lock is poisoned! Please investigate!");
                None
            }
        };

        match data {
            Some(data) => Ok(data),
            None => {
                info!(id = ?id, "guild is not in the cache, fetching from disk");
                self.driver.get_guild(id.get()).await
            }
        }
    }
}
