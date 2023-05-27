use luro_core::{Context, Error};
use poise::samples::register_application_commands_buttons;

use crate::commands::owner::adminabuse::adminabuse;
use crate::commands::owner::nickname::nickname;
use crate::commands::owner::register::register;
use crate::commands::owner::reload::reload;
use crate::commands::owner::save::save;
use crate::commands::owner::shutdown::shutdown;
use crate::commands::owner::massunban::massunban;


mod adminabuse;
mod nickname;
mod register;
mod reload;
mod save;
mod shutdown;
mod massunban;

/// Owner only commands. The bot owner can only execute these, unless you are super, mega special
#[poise::command(
    owners_only,
    prefix_command,
    slash_command,
    category = "Owner",
    subcommands("adminabuse", "shutdown", "register", "reload", "save", "nickname", "massunban")
)]
pub async fn owner(ctx: Context<'_>) -> Result<(), Error> {
    // If you have no slash commands, just call @bot owner to get them to register
    register_application_commands_buttons(ctx).await?;
    Ok(())
}
