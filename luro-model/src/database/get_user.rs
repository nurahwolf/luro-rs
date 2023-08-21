use tracing::{info, warn};
use twilight_model::id::{marker::UserMarker, Id};

use crate::user::LuroUser;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Attempts to get a user from the cache, otherwise gets the user from the database
    pub async fn get_user(&self, id: &Id<UserMarker>) -> anyhow::Result<LuroUser> {
        let data = match self.user_data.read() {
            Ok(data) => data.get(id).cloned(),
            Err(why) => {
                warn!(why = ?why, "user_data lock is poisoned! Please investigate!");
                None
            }
        };

        Ok(match data {
            Some(data) => data,
            None => {
                info!(id = ?id, "user is not in the cache, fetching from disk");
                let data = self.driver.get_user(id.get()).await?;
                match self.user_data.write() {
                    Ok(mut user) => {
                        user.insert(*id, data.clone());
                    }
                    Err(why) => warn!(why = ?why, "user_data lock is poisoned! Please investigate!")
                }
                data
            }
        })
    }
}
