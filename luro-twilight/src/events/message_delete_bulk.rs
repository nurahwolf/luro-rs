use luro_framework::Context;
use luro_model::message::LuroMessageSourceV2;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

pub async fn message_delete_bulk_listener(ctx: Context, event: MessageDeleteBulk) -> anyhow::Result<()> {
    debug!("Messages Bulk Deleted");

    ctx.database
        .update_message(LuroMessageSourceV2::MessageDeleteBulk(event))
        .await?;

    Ok(())
}
