use luro_model::{database_driver::LuroDatabaseDriver, guild::LuroGuild};
use tracing::warn;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{LuroDatabase, LuroDatabaseItem};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Saves a user, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn modify_guild(&self, id: &Id<GuildMarker>, guild: &LuroGuild) -> anyhow::Result<Option<LuroGuild>> {
        let (ok, data) = match self.guild_data.write() {
            Ok(mut data) => (true, Ok(data.insert(*id, guild.clone()))),
            Err(why) => {
                warn!(why = ?why, "guild_data lock is poisoned! Please investigate!");
                (false, Ok(None))
            }
        };

        if ok {
            LuroGuild::modify_item(&id.get(), guild).await?;
        }

        data
    }
}
