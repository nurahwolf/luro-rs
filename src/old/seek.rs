use twilight_lavalink::model::Seek;
use twilight_model::{channel::Message, gateway::payload::incoming::MessageCreate};

use crate::State;

pub async fn seek(msg: Box<MessageCreate>, state: State) -> anyhow::Result<()> {
    tracing::debug!(
        "seek command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );
    state
        .twilight_client
        .create_message(msg.channel_id)
        .content("Where in the track do you want to seek to (in seconds)?")?
        .await?;

    let author_id = msg.author.id;
    let msg = state
        .twilight_standby
        .wait_for_message(msg.channel_id, move |new_msg: &MessageCreate| {
            new_msg.author.id == author_id
        })
        .await?;
    let guild_id = msg.guild_id.unwrap();
    let position = msg.content.parse::<i64>()?;

    let player = state.lavalink.player(guild_id).await.unwrap();
    player.send(Seek::from((guild_id, position * 1000)))?;

    state
        .twilight_client
        .create_message(msg.channel_id)
        .content(&format!("Seeked to {position}s"))?
        .await?;

    Ok(())
}
