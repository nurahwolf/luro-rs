use std::collections::BTreeMap;

use tracing::{info, warn};

use crate::heck::Hecks;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver,> LuroDatabase<D,> {
    /// Attempts to get all hecks from the cache, otherwise gets the user from the database
    pub async fn get_hecks(&self, nsfw: bool,) -> anyhow::Result<Hecks,> {
        let mut data = match self.hecks.read() {
            Ok(data,) => match nsfw {
                true => data.nsfw.clone(),
                false => data.sfw.clone(),
            },
            Err(why,) => {
                warn!(why = ?why, "stories lock is poisoned! Please investigate!");
                BTreeMap::new()
            }
        };

        if data.is_empty() {
            info!("stories are not in the cache, fetching from disk");
            data = self.driver.get_hecks(nsfw,).await?;
            match self.hecks.write() {
                Ok(mut hecks,) => match nsfw {
                    true => hecks.nsfw = data.clone(),
                    false => hecks.sfw = data.clone(),
                },
                Err(why,) => warn!(why = ?why, "hecks lock is poisoned! Please investigate!"),
            }
        }

        Ok(data,)
    }
}
