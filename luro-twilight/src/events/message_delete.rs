use luro_framework::Context;
use luro_model::message::LuroMessageSourceV2;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageDelete;

pub async fn message_delete_listener(ctx: Context, event: MessageDelete) -> anyhow::Result<()> {
    debug!("Message Deleted");

    ctx.database.update_message(LuroMessageSourceV2::MessageDelete(event)).await?;

    Ok(())
}
