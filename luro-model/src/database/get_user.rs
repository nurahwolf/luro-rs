use tracing::{error, info, warn};
use twilight_http::Client;
use twilight_model::id::{marker::UserMarker, Id};

use crate::user::LuroUser;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Attempts to get a user from the cache, otherwise gets the user from the database
    ///
    /// If that fails... Then fetch from Discord's API
    pub async fn get_user(&self, id: &Id<UserMarker>, twilight_client: &Client) -> anyhow::Result<LuroUser> {
        let mut data = match self.user_data.read() {
            Ok(data) => data.get(id).cloned(),
            Err(why) => {
                error!(why = ?why, "user_data lock is poisoned! Please investigate!");
                None
            }
        };

        if data.is_none() {
            info!(id = ?id, "user is not in the cache, fetching from disk");
            data = match self.driver.get_user(id.get()).await {
                Ok(data) => Some(data),
                Err(why) => {
                    warn!(why = ?why, "Failed to get user from the database. Falling back to twilight");
                    None
                }
            }
        }

        let mut response = match data {
            Some(data) => {
                // Flush the new data to disk
                match self.user_data.write() {
                    Ok(mut user) => {
                        user.insert(*id, data.clone());
                    }
                    Err(why) => error!(why = ?why, "user_data lock is poisoned! Please investigate!")
                }
                data
            }
            None => LuroUser::new(*id)
        };

        match twilight_client.user(*id).await {
            Ok(user) => {response.update_user(&user.model().await?);},
            Err(why) => info!(why = ?why, "Failed to update user"),
        }

        Ok(response)
    }
}
