use luro_core::{Context, Error};

use crate::commands::info::channel::channel;
use crate::commands::info::guild::guild;
use crate::commands::info::hecks::hecks;
use crate::commands::info::message::message;
use crate::commands::info::role::role;
use crate::commands::info::user::user;

mod channel;
mod guild;
mod hecks;
mod message;
mod role;
mod user;

/// Get some information on things, like guilds and users.
#[poise::command(
    slash_command,
    category = "Guild",
    subcommands("user", "guild", "channel", "message", "role", "hecks")
)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}
