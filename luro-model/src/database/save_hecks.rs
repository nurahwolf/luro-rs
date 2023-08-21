use tracing::{warn};

use crate::{
    heck::{Hecks}
};

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Modifies multiple hecks, overwriting whatever value used to exist
    pub async fn save_hecks(&self, hecks: Hecks, nsfw: bool) -> anyhow::Result<()> {
        match self.hecks.write() {
            Ok(mut data) => {
                self.driver.modify_hecks(&hecks, nsfw).await?;
                match nsfw {
                    true => data.nsfw = hecks,
                    false => data.sfw = hecks
                }
            }
            Err(why) => warn!(why = ?why, "hecks lock is poisoned! Please investigate!")
        }
        Ok(())
    }
}
