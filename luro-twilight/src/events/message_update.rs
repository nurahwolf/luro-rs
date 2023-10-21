use luro_framework::LuroContext;
use luro_model::message::LuroMessageType;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn message_update_listener(ctx: LuroContext, event: Box<MessageUpdate>) -> anyhow::Result<()> {
    debug!("Message Updated");

    ctx.database.update_message(LuroMessageType::MessageUpdate(*event)).await?;

    Ok(())
}
