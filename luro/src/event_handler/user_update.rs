use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::info;
use twilight_model::gateway::payload::incoming::UserUpdate;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver,> Framework<D,> {
    pub async fn user_update_listener(&self, event: UserUpdate,) -> anyhow::Result<(),> {
        info!("User {} updated", event.id);

        // let mut user = self.database.get_user(&event.id).await?;
        // user.update_user(&event.);
        // self.database.save_user(&event.id, &user).await?;

        Ok((),)
    }
}
