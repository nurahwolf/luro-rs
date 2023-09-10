use std::collections::BTreeMap;

use luro_model::database_driver::LuroDatabaseDriver;
use tracing::{info, warn};

use crate::{Stories, LuroDatabase};


impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Attempts to get stories from the cache, otherwise gets the stories from the database
    pub async fn get_stories(&self, nsfw: bool) -> anyhow::Result<Stories> {
        let mut data = match self.stories.read() {
            Ok(data) => match nsfw {
                true => data.nsfw.clone(),
                false => data.sfw.clone(),
            },
            Err(why) => {
                warn!(why = ?why, "stories lock is poisoned! Please investigate!");
                BTreeMap::new()
            }
        };

        if data.is_empty() {
            info!("stories are not in the cache, fetching from disk");
            data = self.driver.get_stories(nsfw).await?;
            match self.stories.write() {
                Ok(mut stories) => match nsfw {
                    true => stories.nsfw = data.clone(),
                    false => stories.sfw = data.clone(),
                },
                Err(why) => warn!(why = ?why, "stories lock is poisoned! Please investigate!"),
            }
        }

        Ok(data)
    }
}
