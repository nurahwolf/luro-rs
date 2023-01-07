use crate::{Context, Error};
use poise::serenity_prelude::Message;

/// Deletes a particular message sent by the bot.
#[poise::command(
    context_menu_command = "Delete Luro message",
    prefix_command,
    slash_command,
    required_bot_permissions = "MANAGE_MESSAGES",
    category = "Moderation",
    ephemeral
)]
pub async fn delete_botmessage(ctx: Context<'_>, #[description = "Message to be deleted"] msg: Message) -> Result<(), Error> {
    msg.delete(ctx).await?;
    ctx.say("Deleted!").await?;

    Ok(())
}
