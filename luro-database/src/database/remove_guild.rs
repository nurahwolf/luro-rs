use luro_model::database_driver::LuroDatabaseDriver;
use tracing::warn;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::LuroDatabase;


impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Removes a user from the database
    pub async fn remove_guild(&self, id: &Id<GuildMarker>) -> anyhow::Result<()> {
        warn!("Guild {id} was required to be removed from the database!");

        self.driver.remove_guild(id.get()).await
    }
}
