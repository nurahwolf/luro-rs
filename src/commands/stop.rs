use std::sync::Arc;

use twilight_lavalink::model::Stop;
use twilight_model::channel::Message;

use crate::Luro;

pub async fn stop(msg: Message, ctx: Arc<Luro>) -> anyhow::Result<()> {
    tracing::debug!(
        "stop command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );

    let guild_id = msg.guild_id.unwrap();
    let player = ctx.lavalink.player(guild_id).await.unwrap();
    player.send(Stop::from(guild_id))?;

    ctx.http
        .create_message(msg.channel_id)
        .content("Stopped the track")?
        .await?;

    Ok(())
}
