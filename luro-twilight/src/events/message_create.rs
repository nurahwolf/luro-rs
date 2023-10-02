use luro_framework::Context;
use luro_model::message::LuroMessageSourceV2;
use tracing::debug;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn message_create_listener(ctx: Context, event: Box<MessageCreate>) -> anyhow::Result<()> {
    debug!("Message Received");

    ctx.database
        .update_message(LuroMessageSourceV2::MessageCreate(*event))
        .await?;

    Ok(())
}
