use luro_framework::LuroContext;
use luro_model::sync::MessageSync;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

pub async fn message_delete_bulk_listener(ctx: LuroContext, event: MessageDeleteBulk) -> anyhow::Result<()> {
    debug!("Messages Bulk Deleted");

    ctx.database.message_update(MessageSync::MessageDeleteBulk(event)).await?;

    Ok(())
}
