use luro_framework::Context;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageDeleteBulk;

pub async fn message_delete_bulk_listener(ctx: Context, event: MessageDeleteBulk) -> anyhow::Result<()> {
    debug!("Message Deleted");

    for message_id in event.ids {
        if let Some(message) = ctx.cache.message(message_id) {
            ctx.database.update_message(message.clone()).await?;
        }
    }

    Ok(())
}
