use tracing::{info, warn};

use crate::story::Story;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Attempts to get a story from the cache, otherwise gets the user from the database
    pub async fn get_story(&self, id: &usize, nsfw: bool) -> anyhow::Result<Story> {
        let data = match self.stories.read() {
            Ok(data) => match nsfw {
                true => data.nsfw.get(id).cloned(),
                false => data.sfw.get(id).cloned(),
            },

            Err(why) => {
                warn!(why = ?why, "user_data lock is poisoned! Please investigate!");
                None
            }
        };

        match data {
            Some(data) => Ok(data),
            None => {
                info!(id = ?id, "user is not in the cache, fetching from disk");
                self.driver.get_story(id, nsfw).await
            }
        }
    }
}
