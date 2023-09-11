use anyhow::anyhow;
use luro_model::{database_driver::LuroDatabaseDriver, user::LuroUser};
use tracing::{error, info};
use twilight_model::id::{marker::UserMarker, Id};

use crate::{LuroDatabase, LuroDatabaseItem};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Fetches a user in the following priority order
    ///
    /// Luro Cache -> Luro Database -> Twilight Cache -> Twilight Client
    pub async fn get_user(&self, id: &Id<UserMarker>) -> anyhow::Result<LuroUser> {
        // Attempt to fetch the data from Luro's cache
        match self.user_data.read() {
            Ok(data) => {
                if let Some(data) = data.get(id) {
                    return Ok(data.clone());
                }
            }
            Err(why) => error!(why = ?why, "user_data lock is poisoned! Please investigate!"),
        };

        info!(id = ?id, "user is not in Luro's cache, fetching from Luro's Database");
        if let Ok(data) = LuroUser::get_item(&id.get(), ()).await {
            match self.user_data.write() {
                Ok(mut user_data) => {
                    if let Some(data) = user_data.insert(*id, data.clone()) {
                        return Ok(data.clone());
                    }
                }
                Err(why) => error!(why = ?why, "user_data lock is poisoned! Please investigate!"),
            };
            return Ok(data);
        }

        info!(id = ?id, "user is not in Luro's cache, fetching from Twilight's Cache");
        if let Some(data) = self.config.cache.user(*id) {
            let mut user = LuroUser::new(*id);
            user.update_user(&data);
            match self.user_data.write() {
                Ok(mut user_data) => {
                    if let Some(data) = user_data.insert(*id, user.clone()) {
                        return Ok(data.clone());
                    }
                }
                Err(why) => error!(why = ?why, "user_data lock is poisoned! Please investigate!"),
            };
            return Ok(user);
        }

        info!(id = ?id, "user is not in Luro's cache, fetching from Twilight's Client");
        if let Ok(data) = self.config.twilight_client.user(*id).await {
            let mut user = LuroUser::new(*id);
            user.update_user(&data.model().await?);
            match self.user_data.write() {
                Ok(mut user_data) => {
                    if let Some(data) = user_data.insert(*id, user.clone()) {
                        return Ok(data.clone());
                    }
                }
                Err(why) => error!(why = ?why, "user_data lock is poisoned! Please investigate!"),
            };
            return Ok(user);
        }

        Err(anyhow!(
            "Could not find any data relating to the user. Is the user resolvable?"
        ))
    }
}
