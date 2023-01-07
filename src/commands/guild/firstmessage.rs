use poise::serenity_prelude::Channel;

use crate::{Context, Error, functions::guild_accent_colour::guild_accent_colour};

/// Retrieves the first message ever sent to a channel.
#[poise::command(slash_command, prefix_command, guild_only, required_permissions = "READ_MESSAGE_HISTORY")]
pub async fn firstmessage(ctx: Context<'_>, #[description = "The channel"] channel: Channel) -> Result<(), Error> {
    let channel_id = channel.id();
    let messages = channel_id.messages(ctx, |retriever| retriever.after(1).limit(1)).await.unwrap();
    let msg = messages.first().unwrap();
    let msg_link = msg.link();

    ctx.send(|message| {
        message.embed(|embed| {
            embed.author(|a| a.name(msg.author.tag()).icon_url(msg.author.avatar_url().unwrap()));
            embed.colour(guild_accent_colour(ctx.data().config.lock().unwrap().accent_colour, ctx.guild()));
            embed.thumbnail(msg.author.avatar_url().unwrap());
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
