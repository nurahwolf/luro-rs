use crate::{Command, Context, Error};

mod punish;
mod purge;

/// Moderator Commands :)
#[poise::command(slash_command, category = "Guild", subcommands())]
pub async fn moderator(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}

pub fn commands() -> [Command; 3] {
    [purge::purge(), punish::punish(), moderator()]
}
