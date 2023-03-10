use luro_core::{Context, Error};

use crate::commands::context_commands::delete_bot_message::delete_bot_message;
use crate::commands::context_commands::saucenao::saucenao;
use crate::commands::context_commands::user_info::userinfo;

mod delete_bot_message;
mod saucenao;
mod user_info;

/// These commands run in context menus only.
#[poise::command(
    slash_command,
    category = "Context Menu",
    subcommands("delete_bot_message", "saucenao", "userinfo")
)]
pub async fn context_commands(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}
