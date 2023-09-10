use luro_model::{database_driver::LuroDatabaseDriver, heck::Heck};
use tracing::warn;

use crate::{LuroDatabase, LuroDatabaseItem};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Modifies a heck, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn save_heck(&self, id: usize, heck: Heck) -> anyhow::Result<Option<Heck>> {
        let (ok, data) = match self.hecks.write() {
            Ok(mut data) => (
                true,
                match heck.nsfw {
                    true => Ok(data.nsfw.insert(id, heck.clone())),
                    false => Ok(data.sfw.insert(id, heck.clone())),
                },
            ),
            Err(why) => {
                warn!(why = ?why, "hecks lock is poisoned! Please investigate!");
                (false, Ok(None))
            }
        };

        if ok {
            Heck::add_item(&heck).await?;
        }

        data
    }
}
