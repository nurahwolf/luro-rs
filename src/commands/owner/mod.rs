use crate::{
    commands::owner::{command_config_reload::reload, command_config_save::save},
    Command, Context, Error
};

mod command_adminabuse;
mod command_config_reload;
mod command_config_save;
mod command_register;
mod command_shutdown;

#[poise::command(owners_only, slash_command, category = "Owner", subcommands("reload", "save"))]
pub async fn config(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}

pub fn commands() -> [Command; 4] {
    [command_adminabuse::adminabuse(), command_register::register(), config(), command_shutdown::shutdown()]
}
