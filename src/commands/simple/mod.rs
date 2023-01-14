use crate::Command;

mod about;
mod command_usage;
mod embed;
mod help;
mod invite;
mod ping;
mod say;

pub fn commands() -> [Command; 7] {
    [
        about::about(),
        command_usage::command_usage(),
        embed::embed(),
        help::help(),
        invite::invite(),
        ping::ping(),
        say::say()
    ]
}
