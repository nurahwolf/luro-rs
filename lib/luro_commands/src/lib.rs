#![feature(let_chains)]

use luro_core::Command;
use luro_dice::dice_commands;
use luro_e621::e621_commands;
use luro_furaffinity::furaffinity_commands;
use luro_sled::sled_commands;
use luro_songbird::commands::songbird_commands;

mod commands;
mod functions;
mod structs;

pub fn commands() -> Vec<Command> {
    commands::commands()
        .into_iter()
        .chain(songbird_commands())
        .chain(e621_commands())
        .chain(furaffinity_commands())
        .chain(sled_commands())
        .chain(dice_commands())
        .collect()
}
