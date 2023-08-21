use tracing::warn;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::guild::LuroGuild;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Saves a user, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn save_guild(&self, id: &Id<GuildMarker>, guild: &LuroGuild) -> anyhow::Result<Option<LuroGuild>> {
        let (ok, data) = match self.guild_data.write() {
            Ok(mut data) => (true, Ok(data.insert(*id, guild.clone()))),
            Err(why) => {
                warn!(why = ?why, "guild_data lock is poisoned! Please investigate!");
                (false, Ok(None))
            }
        };

        if ok {
            self.driver.save_guild(id.get(), guild.clone()).await?;
        }

        data
    }
}
