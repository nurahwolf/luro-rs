use luro_core::{Context, Error};

use crate::commands::config::reload::reload;
use crate::commands::config::save::save;

mod reload;
mod save;

#[poise::command(owners_only, slash_command, category = "Owner", subcommands("reload", "save"))]
pub async fn config(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}
