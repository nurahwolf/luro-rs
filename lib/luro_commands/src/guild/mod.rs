use luro_core::{Command, Context, Error};

use crate::guild::{info_channel::channel, info_guild::guild, info_message::message, info_role::role, info_user::user};

mod firstmessage;
mod guilds;
mod info_channel;
mod info_guild;
mod info_message;
mod info_role;
mod info_user;

/// Get some information on things, like guilds and users.
#[poise::command(
    slash_command,
    category = "Guild",
    subcommands("user", "guild", "channel", "message", "role")
)]
pub async fn info(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("This command only has subcommands I'm afraid :)").await?;
    Ok(())
}

pub fn commands() -> [Command; 4] {
    [
        firstmessage::firstmessage(),
        guilds::guilds(),
        info(),
        info_user::userinfo_context()
    ]
}