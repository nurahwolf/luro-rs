use crate::Command;

mod command_usage;
mod embed;
mod help;
mod ping;
mod say;

pub fn commands() -> [Command; 5] {
    [
        command_usage::command_usage(),
        embed::embed(),
        help::help(),
        ping::ping(),
        say::say()
    ]
}
