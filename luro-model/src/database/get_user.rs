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
        let data = match self.user_data.read() {
            Ok(data) => data.get(id).cloned(),
            Err(why) => {
                warn!(why = ?why, "user_data lock is poisoned! Please investigate!");
                None
            }
        };

        let user = twilight_client.user(*id).await;

        Ok(match data {
            Some(mut data) => {
                if let Ok(user) = user {
                    data.update_user(&user.model().await?);
                }
                data
            }
            None => {
                info!(id = ?id, "user is not in the cache, fetching from disk");
                let mut data = match self.driver.get_user(id.get()).await {
                    Ok(data) => data,
                    Err(why) => {
                        error!(why = ?why, "Failed to get user from the database. Falling back to twilight");
                        LuroUser::new(*id)
                    }
                };
                if let Ok(user) = user {
                    data.update_user(&user.model().await?);
                }
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
