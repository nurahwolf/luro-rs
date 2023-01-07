use crate::Command;

mod embed;
mod help;
mod invite;
mod ping;
mod say;

pub fn commands() -> [Command; 5] {
    [embed::embed(), help::help(), invite::invite(), ping::ping(), say::say()]
}
