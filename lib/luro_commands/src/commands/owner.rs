use luro_core::{Context, Error};
use poise::samples::register_application_commands_buttons;

use crate::commands::owner::adminabuse::adminabuse;
use crate::commands::owner::register::register;
use crate::commands::owner::shutdown::shutdown;

mod adminabuse;
mod register;
mod shutdown;

/// Owner only commands. The bot owner can only execute these, unless you are super, mega special
#[poise::command(
    owners_only,
    prefix_command,
    slash_command,
    category = "Owner",
    subcommands("adminabuse", "shutdown", "register")
)]
pub async fn owner(ctx: Context<'_>) -> Result<(), Error> {
    // If you have no slash commands, just call @bot owner to get them to register
    register_application_commands_buttons(ctx).await?;
    Ok(())
}
