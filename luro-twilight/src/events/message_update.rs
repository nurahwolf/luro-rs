use luro_framework::LuroContext;
use luro_model::sync::MessageSync;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn message_update_listener(ctx: LuroContext, event: Box<MessageUpdate>) -> anyhow::Result<()> {
    debug!("Message Updated");

    ctx.database.message_update(MessageSync::MessageUpdate(*event)).await?;

    Ok(())
}
