use tracing::warn;
use twilight_model::id::{marker::UserMarker, Id};

use super::{drivers::LuroDatabaseDriver, LuroDatabase};

impl<D: LuroDatabaseDriver> LuroDatabase<D> {
    /// Removes a user from the database
    pub async fn remove_user(&self, id: &Id<UserMarker>) -> anyhow::Result<()> {
        warn!("User {id} was required to be removed from the database!");

        self.driver.remove_user(id.get()).await
    }
}
