use luro_framework::LuroContext;
use tracing::info;
use twilight_model::gateway::payload::incoming::UserUpdate;
pub async fn user_update_listener(ctx: LuroContext, event: UserUpdate) -> anyhow::Result<()> {
    info!("User {} updated", event.id);

    ctx.database.user_update(event).await?;

    Ok(())
}