use luro_core::{Context, Error};
use poise::serenity_prelude::Message;

/// Deletes a particular message sent by the bot.
#[poise::command(context_menu_command = "Delete message by bot", category = "Luro", ephemeral)]
pub async fn delete_bot_message(ctx: Context<'_>, #[description = "Message to be deleted"] msg: Message) -> Result<(), Error> {
    match msg.delete(ctx).await {
        Ok(_) => ctx.say("Deleted!").await?,
        Err(err) => ctx.say(format!("Failed to delete that message, probably because I don't have permissions. The error was: {err}")).await?,
    };
    
    Ok(())
}
