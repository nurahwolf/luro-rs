use twilight_lavalink::model::Pause;
use twilight_model::{channel::Message, gateway::payload::incoming::MessageCreate};

use crate::State;

pub async fn pause(msg: Box<MessageCreate>, state: State) -> anyhow::Result<()> {
    tracing::debug!(
        "pause command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );

    let guild_id = msg.guild_id.unwrap();
    let player = state.lavalink.player(guild_id).await.unwrap();
    let paused = player.paused();
    player.send(Pause::from((guild_id, !paused)))?;

    let action = if paused { "Unpaused " } else { "Paused" };

    state
        .twilight_client
        .create_message(msg.channel_id)
        .content(&format!("{action} the track"))?
        .await?;

    Ok(())
}
