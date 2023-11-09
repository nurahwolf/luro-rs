use luro_framework::LuroContext;
use luro_model::sync::MessageSync;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn message_create_listener(ctx: LuroContext, event: Box<MessageCreate>) -> anyhow::Result<()> {
    debug!("Message Received");

    ctx.database.message_update(MessageSync::MessageCreate(*event)).await?;

    Ok(())
}
