use std::sync::Arc;

use twilight_gateway::{MessageSender, stream::ShardRef};
use twilight_lavalink::model::Destroy;
use twilight_model::{channel::Message, gateway::payload::{outgoing::UpdateVoiceState, incoming::MessageCreate}};

use crate::State;

pub async fn leave(msg: Box<MessageCreate>, state: State, mut shard: ShardRef<'_>) -> anyhow::Result<()> {
    tracing::debug!(
        "leave command in channel {} by {}",
        msg.channel_id,
        msg.author.name
    );

    let guild_id = msg.guild_id.unwrap();
    let player = state.lavalink.player(guild_id).await.unwrap();
    player.send(Destroy::from(guild_id))?;
    shard.command(&UpdateVoiceState::new(guild_id, None, false, false));

    state
        .twilight_client
        .create_message(msg.channel_id)
        .content("Left the channel")?
        .await?;

    Ok(())
}
