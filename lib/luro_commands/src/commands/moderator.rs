use luro_core::{Context, Error};

use crate::commands::moderator::punish::punish;
use crate::commands::moderator::purge::purge;
use crate::commands::moderator::settings::settings;

mod punish;
mod purge;
mod settings;

/// Moderator only subcommands. These commands check for permissions.
#[poise::command(slash_command, category = "Moderator", subcommands("purge", "punish", "settings"))]
pub async fn moderator(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}
