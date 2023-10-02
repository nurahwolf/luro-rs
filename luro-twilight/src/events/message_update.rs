use luro_framework::Context;
use luro_model::message::LuroMessageType;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn message_update_listener(ctx: Context, event: Box<MessageUpdate>) -> anyhow::Result<()> {
    debug!("Message Updated");

    ctx.database
        .update_message(LuroMessageType::MessageUpdate(*event))
        .await?;

    Ok(())
}
