use std::sync::Arc;

use twilight_lavalink::model::Pause;
use twilight_model::channel::Message;

use crate::Luro;

pub async fn pause(msg: Message, ctx: Arc<Luro>) -> anyhow::Result<()> {
    tracing::debug!(
        "pause command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );

    let guild_id = msg.guild_id.unwrap();
    let player = ctx.lavalink.player(guild_id).await.unwrap();
    let paused = player.paused();
    player.send(Pause::from((guild_id, !paused)))?;

    let action = if paused { "Unpaused " } else { "Paused" };

    ctx.http
        .create_message(msg.channel_id)
        .content(&format!("{action} the track"))?
        .await?;

    Ok(())
}
