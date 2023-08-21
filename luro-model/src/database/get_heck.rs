use tracing::{info, warn};

use crate::heck::Heck;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Attempts to get a heck from the cache, otherwise gets the user from the database
    pub async fn get_heck(&self, id: &usize, nsfw: bool) -> anyhow::Result<Option<Heck>> {
        let data = match self.hecks.read() {
            Ok(data) => match nsfw {
                true => data.nsfw.get(id).cloned(),
                false => data.sfw.get(id).cloned()
            },
            Err(why) => {
                warn!(why = ?why, "hecks lock is poisoned! Please investigate!");
                None
            }
        };

        Ok(match data {
            Some(data) => Some(data),
            None => {
                info!("hecks are not in the cache, fetching from disk");
                let data = self.driver.get_hecks(nsfw).await?;
                match self.hecks.write() {
                    Ok(mut hecks) => match nsfw {
                        true => hecks.nsfw = data.clone(),
                        false => hecks.sfw = data.clone()
                    },
                    Err(why) => warn!(why = ?why, "hecks lock is poisoned! Please investigate!")
                }
                data.get(id).cloned()
            }
        })
    }
}
