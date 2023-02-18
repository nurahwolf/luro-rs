use std::sync::Arc;

use twilight_gateway::MessageSender;
use twilight_model::{
    channel::Message,
    gateway::payload::{incoming::MessageCreate, outgoing::UpdateVoiceState},
};

use crate::Luro;

pub async fn join(msg: Message, ctx: Arc<Luro>, shard: Arc<MessageSender>) -> anyhow::Result<()> {
    ctx.http
        .create_message(msg.channel_id)
        .content("What's the channel ID you want me to join?")?
        .await?;

    let author_id = msg.author.id;
    let msg = ctx
        .standby
        .wait_for_message(msg.channel_id, move |new_msg: &MessageCreate| {
            new_msg.author.id == author_id
        })
        .await?;
    let channel_id = msg.content.parse()?;
    let guild_id = msg.guild_id.expect("known to be present");

    shard.command(&UpdateVoiceState::new(
        guild_id,
        Some(channel_id),
        false,
        false,
    ))?;

    ctx.http
        .create_message(msg.channel_id)
        .content(&format!("Joined <#{channel_id}>!"))?
        .await?;

    Ok(())
}
