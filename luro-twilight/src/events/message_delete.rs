use luro_framework::Context;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageDelete;

pub async fn message_delete_listener(ctx: Context, event: MessageDelete) -> anyhow::Result<()> {
    debug!("Message Deleted");

    ctx.database.update_message(event).await?;

    Ok(())
}
