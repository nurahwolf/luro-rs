use tracing::{warn};

use crate::{Stories};

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Modifies multiple hecks, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn save_stories(&self, stories: Stories, nsfw: bool) -> anyhow::Result<()> {
        match self.stories.write() {
            Ok(mut data) => {
                self.driver.modify_stories(&stories, nsfw).await?;
                match nsfw {
                    true => data.nsfw = stories,
                    false => data.sfw = stories
                }
            }
            Err(why) => warn!(why = ?why, "stories lock is poisoned! Please investigate!")
        }
        Ok(())
    }
}
