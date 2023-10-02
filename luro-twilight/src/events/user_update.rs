use luro_framework::Context;
use tracing::info;
use twilight_model::gateway::payload::incoming::UserUpdate;
pub async fn user_update_listener(ctx: Context, event: UserUpdate) -> anyhow::Result<()> {
    info!("User {} updated", event.id);

    ctx.database.update_user(event).await?;

    Ok(())
}
