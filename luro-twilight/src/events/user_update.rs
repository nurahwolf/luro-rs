use luro_framework::Context;
use tracing::info;
use twilight_model::gateway::payload::incoming::UserUpdate;
pub async fn user_update_listener(_ctx: Context, event: UserUpdate) -> anyhow::Result<()> {
    info!("User {} updated", event.id);

    // let mut user = self.database.get_user(&event.id).await?;
    // user.update_user(&event.);
    // self.database.save_user(&event.id, &user).await?;

    Ok(())
}
