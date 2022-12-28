use crate::Command;

mod command_embed;
mod command_help;
mod command_invite;
mod command_ping;
mod command_say;

pub fn commands() -> [Command; 5] {
    [command_embed::embed(), command_help::help(), command_invite::invite(), command_ping::ping(), command_say::say()]
}
