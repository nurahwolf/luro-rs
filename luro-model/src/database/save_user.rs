use tracing::warn;
use twilight_model::id::{marker::UserMarker, Id};

use crate::user::LuroUser;

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver,> LuroDatabase<D,> {
    /// Saves a user, overwriting whatever value used to exist
    /// Returns the old users data if it existed
    pub async fn save_user(&self, id: &Id<UserMarker,>, user: &LuroUser,) -> anyhow::Result<Option<LuroUser,>,> {
        let (ok, data,) = match self.user_data.write() {
            Ok(mut data,) => (true, Ok(data.insert(*id, user.clone(),),),),
            Err(why,) => {
                warn!(why = ?why, "user_data lock is poisoned! Please investigate!");
                (false, Ok(None,),)
            }
        };

        if ok {
            self.driver.save_user(id.get(), user,).await?;
        }

        data
    }
}
