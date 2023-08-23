use anyhow::Context;
use tracing::{error, info, warn};
use twilight_cache_inmemory::InMemoryCache;
use twilight_model::id::{marker::UserMarker, Id};

use crate::user::LuroUser;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Attempts to get a user from the cache, otherwise gets the user from the database
    ///
    /// If that fails... Then fetch from Discord's API
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

        let mut data = match data {
            Some(data) => data,
            None => LuroUser::new(*id)
        };

        match self.twilight_client.user(*id).await {
            Ok(user) => {
                data.update_user(&user.model().await?);
            }
            Err(why) => info!(why = ?why, "Failed to update user - {id}")
        }

        match self.user_data.write() {
            Ok(mut user_data) => {
                user_data.insert(*id, data.clone());
            }
            Err(why) => error!(why = ?why, "user_data lock is poisoned! Please investigate!")
        }

        Ok(data)
    }

    pub async fn get_user_cached(&self, id: &Id<UserMarker>, cache: &InMemoryCache) -> anyhow::Result<LuroUser> {
        let cached_user = cache
            .user(*id)
            .map(|x| x.clone())
            .context("Expected to get user from cache")?;
        let mut data = match self.user_data.read() {
            Ok(data) => data.get(id).cloned(),
            Err(why) => {
                error!(why = ?why, "user_data lock is poisoned! Please investigate!");
                None
            }
        };

        if data.is_none() {
            info!(id = ?id, "(Cached) user is not in the cache, fetching from disk");
            data = match self.driver.get_user(id.get()).await {
                Ok(data) => Some(data),
                Err(why) => {
                    warn!(why = ?why, "Failed to get user from the database. Falling back to twilight");
                    None
                }
            }
        }

        let mut data = match data {
            Some(data) => data,
            None => LuroUser::new(*id)
        };

        data.update_user(&cached_user);

        match self.user_data.write() {
            Ok(mut user_data) => {
                user_data.insert(*id, data.clone());
            }
            Err(why) => error!(why = ?why, "user_data lock is poisoned! Please investigate!")
        }

        Ok(data)
    }
}
