use twilight_lavalink::model::Volume;
use twilight_model::{channel::Message, gateway::payload::incoming::MessageCreate};

use crate::State;

pub async fn volume(msg: Box<MessageCreate>, state: State) -> anyhow::Result<()> {
    tracing::debug!(
        "volume command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );
    state
        .twilight_client
        .create_message(msg.channel_id)
        .content("What's the volume you want to set (0-1000, 100 being the default)?")?
        .await?;

    let author_id = msg.author.id;
    let msg = state
        .twilight_standby
        .wait_for_message(msg.channel_id, move |new_msg: &MessageCreate| {
            new_msg.author.id == author_id
        })
        .await?;
    let guild_id = msg.guild_id.unwrap();
    let volume = msg.content.parse::<i64>()?;

    if !(0..=1000).contains(&volume) {
        state
            .twilight_client
            .create_message(msg.channel_id)
            .content("That's more than 1000")?
            .await?;

        return Ok(());
    }

    let player = state.lavalink.player(guild_id).await.unwrap();
    player.send(Volume::from((guild_id, volume)))?;

    state
        .twilight_client
        .create_message(msg.channel_id)
        .content(&format!("Set the volume to {volume}"))?
        .await?;

    Ok(())
}
