use luro_model::{database_driver::LuroDatabaseDriver, heck::Hecks};
use tracing::warn;

use crate::LuroDatabase;


impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Modifies multiple hecks, overwriting whatever value used to exist
    pub async fn save_hecks(&self, hecks: Hecks, nsfw: bool) -> anyhow::Result<()> {
        let ok = match self.hecks.write() {
            Ok(mut data) => {
                match nsfw {
                    true => data.nsfw = hecks.clone(),
                    false => data.sfw = hecks.clone(),
                }
                true
            }
            Err(why) => {
                warn!(why = ?why, "hecks lock is poisoned! Please investigate!");
                false
            }
        };

        if ok {
            self.driver.modify_hecks(&hecks, nsfw).await?;
        }

        Ok(())
    }
}
