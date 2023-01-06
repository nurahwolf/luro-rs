use crate::{
    commands::guild::{command_info_channel::channel, command_info_guild::guild, command_info_message::message, command_info_user::user},
    Command, Context, Error
};

mod command_botnick;
mod command_firstmessage;
mod command_guilds;
mod command_info_channel;
mod command_info_guild;
mod command_info_message;
mod command_info_user;
mod function_sortuserroles;

/// Get some information on things, like guilds and users.
#[poise::command(slash_command, category = "Guild", subcommands("user", "guild", "channel", "message"))]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}

pub fn commands() -> [Command; 5] {
    [
        command_botnick::botnick(),
        command_firstmessage::firstmessage(),
        command_guilds::guilds(),
        info(),
        command_info_user::userinfo_context()
    ]
}
