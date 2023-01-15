use poise::serenity_prelude::Channel;

use crate::{functions::guild_accent_colour::guild_accent_colour, Context, Error};

/// Retrieves the first message ever sent to a channel.
#[poise::command(slash_command, prefix_command, guild_only, required_permissions = "READ_MESSAGE_HISTORY")]
pub async fn firstmessage(ctx: Context<'_>, #[description = "The channel"] channel: Channel) -> Result<(), Error> {
    let channel_id = channel.id();
    let messages = match channel_id.messages(ctx, |retriever| retriever.after(1).limit(1)).await {
        Ok(ok) => ok,
        Err(err) => {
            ctx.say(format!("Failed to get messages because `{err}`")).await?;
            return Ok(());
        }
    };
    let msg = match messages.first() {
        Some(ok) => ok,
        None => {
            ctx.say("Failed to find any messages").await?;
            return Ok(());
        }
    };
    let msg_link = msg.link();
    let accent_colour = ctx.data().config.read().await.accent_colour;

    ctx.send(|message| {
        message.embed(|embed| {
            embed.author(|a| a.name(msg.author.tag()).icon_url(msg.author.avatar_url().unwrap_or_default()));
            embed.colour(guild_accent_colour(accent_colour, ctx.guild()));
            embed.thumbnail(msg.author.avatar_url().unwrap_or_default());
            embed.description(&msg.content);
            embed.timestamp(msg.timestamp);
            embed.field("❯ Jump To Message", format!("[Click Here]({msg_link})"), true);
            embed.footer(|f| f.text(format!("Message ID: {}", msg.id)));
            embed
        })
    })
    .await?;

    Ok(())
}
