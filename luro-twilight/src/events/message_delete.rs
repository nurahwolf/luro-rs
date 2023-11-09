use luro_framework::LuroContext;
use luro_model::sync::MessageSync;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageDelete;

pub async fn message_delete_listener(ctx: LuroContext, event: MessageDelete) -> anyhow::Result<()> {
    debug!("Message Deleted");

    ctx.database.message_update(MessageSync::MessageDelete(event)).await?;

    Ok(())
}
