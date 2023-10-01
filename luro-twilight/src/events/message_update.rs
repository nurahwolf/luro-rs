use luro_framework::Context;
use luro_model::message::LuroMessageSourceV2;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn message_update_listener(ctx: Context, event: Box<MessageUpdate>) -> anyhow::Result<()> {
    debug!("Message Updated");

    ctx.database.update_message(LuroMessageSourceV2::MessageUpdate(*event)).await?;

    Ok(())
}
