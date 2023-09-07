use std::sync::Arc;

use luro_framework::{Context, Framework};
use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::info;
use twilight_model::gateway::payload::incoming::UserUpdate;

pub async fn user_update_listener<D: LuroDatabaseDriver>(
    _framework: Arc<Framework<D>>,
    _ctx: Context,
    event: UserUpdate
) -> anyhow::Result<()> {
    info!("User {} updated", event.id);

    // let mut user = self.database.get_user(&event.id).await?;
    // user.update_user(&event.);
    // self.database.save_user(&event.id, &user).await?;

    Ok(())
}
