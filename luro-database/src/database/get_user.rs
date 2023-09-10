use luro_model::{user::LuroUser, database_driver::LuroDatabaseDriver};
use tracing::{error, info, warn};
use twilight_model::id::{marker::UserMarker, Id};

use crate::LuroDatabase;

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    pub async fn get_user(&self, id: &Id<UserMarker>) -> anyhow::Result<LuroUser> {
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

        // // If user wants fresh data, or the database did not have data...
        // if fetch_user || data.is_none() {
        //     info!("Fetching data for {id} at request / not available in database");
        //     let mut data = LuroUser::new(*id);
        //     match self.config.twilight_client.user(*id).await {
        //         Ok(user) => {
        //             data.update_user(&user.model().await?);
        //         }
        //         Err(why) => info!(why = ?why, "Failed to update user - {id}"),
        //     }

        //     match self.user_data.write() {
        //         Ok(mut user_data) => {
        //             user_data.insert(*id, data.clone());
        //         }
        //         Err(why) => error!(why = ?why, "user_data lock is poisoned! Please investigate!"),
        //     }
        //     Ok(data)
        // } else {
        //     // Safe to unwrap due to the previous check
        //     Ok(data.unwrap())
        // }

        Ok(data.unwrap())
    }
}
