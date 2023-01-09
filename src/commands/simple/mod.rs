use crate::Command;

mod command_usage;
mod embed;
mod help;
mod invite;
mod ping;
mod say;

pub fn commands() -> [Command; 6] {
    [command_usage::command_usage(), embed::embed(), help::help(), invite::invite(), ping::ping(), say::say()]
}
