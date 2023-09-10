use luro_model::{database_driver::LuroDatabaseDriver, user::LuroUser};
use tracing::warn;
use twilight_model::id::{marker::UserMarker, Id};

use crate::{LuroDatabase, LuroDatabaseItem};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Removes a user from the database
    pub async fn remove_user(&self, id: &Id<UserMarker>) -> anyhow::Result<Option<LuroUser>> {
        warn!("User {id} was required to be removed from the database!");

        LuroUser::remove_item(&id.get(), ()).await
    }
}
