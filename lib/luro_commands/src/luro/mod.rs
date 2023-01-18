use crate::luro::about::{about, about_bot};
use crate::luro::cleanup::cleanup;
use crate::luro::invite::invite;
use crate::luro::nickname::nickname;
use luro_core::{Command, Context, Error};
use poise::serenity_prelude::Message;

mod about;
mod cleanup;
mod invite;
mod nickname;

/// Luro specific commands
#[poise::command(slash_command, category = "Luro", subcommands("cleanup", "nickname", "about", "invite"))]
pub async fn luro(ctx: Context<'_>) -> Result<(), Error> {
    about_bot(ctx).await?;

    Ok(())
}

/// Deletes a particular message sent by the bot.
#[poise::command(context_menu_command = "Delete message by bot", category = "Luro", ephemeral)]
pub async fn delete_message_by_bot(
    ctx: Context<'_>,
    #[description = "Message to be deleted"] msg: Message
) -> Result<(), Error> {
    msg.delete(ctx).await?;
    ctx.say("Deleted!").await?;

    Ok(())
}

pub fn commands() -> [Command; 3] {
    [cleanup::cleanup(), luro(), delete_message_by_bot()]
}
