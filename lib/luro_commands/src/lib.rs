#![feature(let_chains)]
#![feature(option_result_contains)]

use luro_core::Command;
use luro_e621::e621_commands;
use luro_furaffinity::furaffinity_commands;
use luro_songbird::commands::songbird_commands;

mod api;
pub mod furry;
mod guild;
mod luro;
mod moderator;
mod owner;
mod quote;
mod silly;
mod simple;
mod testing;
mod structs;
mod functions;

pub fn commands() -> Vec<Command> {
    songbird_commands().into_iter()
        .chain(e621_commands())
        .chain(furaffinity_commands())
        .chain(owner::commands())
        .chain(simple::commands())
        .chain(moderator::commands())
        .chain(guild::commands())
        .chain(quote::commands())
        .chain(furry::commands())
        .chain(silly::commands())
        .chain(api::commands())
        .chain(testing::commands())
        .chain(luro::commands())
        .collect()
}