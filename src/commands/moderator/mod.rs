use crate::{Command, Context, Error};

mod command_cleanup;
mod command_deletebotmessage;
mod command_punish;

/// Moderator Commands :)
#[poise::command(slash_command, category = "Guild", subcommands())]
pub async fn moderator(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}

pub fn commands() -> [Command; 4] {
    [command_cleanup::cleanup(), command_deletebotmessage::delete_botmessage(), command_punish::punish(), moderator()]
}
