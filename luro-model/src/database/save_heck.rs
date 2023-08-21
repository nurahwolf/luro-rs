use tracing::warn;

use crate::heck::Heck;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Modifies a heck, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn save_heck(&self, id: usize, heck: Heck, nsfw: bool) -> anyhow::Result<Option<Heck>> {
        let (ok, data) = match self.hecks.write() {
            Ok(mut data) => (
                true,
                match nsfw {
                    true => Ok(data.nsfw.insert(id, heck.clone())),
                    false => Ok(data.sfw.insert(id, heck.clone()))
                }
            ),
            Err(why) => {
                warn!(why = ?why, "hecks lock is poisoned! Please investigate!");
                (false, Ok(None))
            }
        };

        if ok {
            self.driver.modify_heck(id, &heck, nsfw).await?;
        }

        data
    }
}
