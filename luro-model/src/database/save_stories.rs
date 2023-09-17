use tracing::warn;

use crate::{
    database_driver::{LuroDatabase, LuroDatabaseDriver},
    Stories,
};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Modifies multiple hecks, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn save_stories(&self, stories: Stories, nsfw: bool) -> anyhow::Result<()> {
        let ok = match self.stories.write() {
            Ok(mut data) => {
                match nsfw {
                    true => data.nsfw = stories.clone(),
                    false => data.sfw = stories.clone(),
                }
                true
            }
            Err(why) => {
                warn!(why = ?why, "stories lock is poisoned! Please investigate!");
                false
            }
        };

        if ok {
            self.driver.modify_stories(&stories, nsfw).await?;
        }

        Ok(())
    }
}
