use luro_model::{database_driver::LuroDatabaseDriver, story::Story};
use tracing::warn;

use crate::LuroDatabase;

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Modifies a story, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn save_story(&self, id: usize, story: Story, nsfw: bool) -> anyhow::Result<Option<Story>> {
        let (ok, data) = match self.stories.write() {
            Ok(mut data) => (
                true,
                match nsfw {
                    true => Ok(data.nsfw.insert(id, story.clone())),
                    false => Ok(data.sfw.insert(id, story.clone())),
                },
            ),
            Err(why) => {
                warn!(why = ?why, "stories lock is poisoned! Please investigate!");
                (false, Ok(None))
            }
        };

        if ok {
            self.driver.save_story(&story, nsfw).await?;
        }

        data
    }
}
