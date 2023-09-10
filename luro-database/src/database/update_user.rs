use luro_model::{database_driver::LuroDatabaseDriver, user::LuroUser};
use tracing::warn;
use twilight_model::id::{marker::UserMarker, Id};

use crate::{LuroDatabase, LuroDatabaseItem};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Saves a user, overwriting whatever value used to exist
    /// Returns the old users data if it existed in the cache
    pub async fn modify_user(&self, id: &Id<UserMarker>, user: &LuroUser) -> anyhow::Result<Option<LuroUser>> {
        let (ok, data) = match self.user_data.write() {
            Ok(mut data) => (true, Ok(data.insert(*id, user.clone()))),
            Err(why) => {
                warn!(why = ?why, "user_data lock is poisoned! Please investigate!");
                (false, Ok(None))
            }
        };

        if ok {
            LuroUser::modify_item(&id.get(), user).await?;
        }

        data
    }
}
