use tracing::warn;

use crate::heck::Heck;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Modifies a heck, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn save_heck(&self, id: usize, heck: Heck, nsfw: bool) -> anyhow::Result<Option<Heck>> {
        match self.hecks.write() {
            Ok(mut data) => {
                self.driver.modify_heck(id, &heck, nsfw).await?;
                match nsfw {
                    true => Ok(data.nsfw.insert(id, heck)),
                    false => Ok(data.sfw.insert(id, heck))
                }
            }
            Err(why) => {
                warn!(why = ?why, "hecks lock is poisoned! Please investigate!");
                Ok(None)
            }
        }
    }
}
