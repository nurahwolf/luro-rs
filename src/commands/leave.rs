use std::sync::Arc;

use twilight_gateway::MessageSender;
use twilight_lavalink::model::Destroy;
use twilight_model::{channel::Message, gateway::payload::outgoing::UpdateVoiceState};

use crate::Luro;

pub async fn leave(msg: Message, ctx: Arc<Luro>, shard: Arc<MessageSender>) -> anyhow::Result<()> {
    tracing::debug!(
        "leave command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );

    let guild_id = msg.guild_id.unwrap();
    let player = ctx.lavalink.player(guild_id).await.unwrap();
    player.send(Destroy::from(guild_id))?;
    shard.command(&UpdateVoiceState::new(guild_id, None, false, false))?;

    ctx.http
        .create_message(msg.channel_id)
        .content("Left the channel")?
        .await?;

    Ok(())
}
