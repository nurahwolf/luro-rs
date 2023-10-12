use luro_framework::Context;
use luro_model::message::LuroMessageType;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

pub async fn message_delete_bulk_listener(ctx: Context, event: MessageDeleteBulk) -> anyhow::Result<()> {
    debug!("Messages Bulk Deleted");

    ctx.database.update_message(LuroMessageType::MessageDeleteBulk(event)).await?;

    Ok(())
}
