use luro_core::{Command, Context, Error};

use crate::owner::{config_reload::reload, config_save::save};

mod adminabuse;
mod config_reload;
mod config_save;
mod register;
mod shutdown;

#[poise::command(owners_only, slash_command, category = "Owner", subcommands("reload", "save"))]
pub async fn config(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}

pub fn commands() -> [Command; 4] {
    [adminabuse::adminabuse(), register::register(), config(), shutdown::shutdown()]
}
