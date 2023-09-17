use tracing::warn;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{
    database_driver::{LuroDatabase, LuroDatabaseDriver},
    guild::LuroGuild,
    luro_database_driver::LuroDatabaseItem,
};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Removes a user from the database
    pub async fn remove_guild(&self, id: &Id<GuildMarker>) -> anyhow::Result<Option<LuroGuild>> {
        warn!("Guild {id} was required to be removed from the database!");

        LuroGuild::remove_item(&id.get(), ()).await
    }
}
