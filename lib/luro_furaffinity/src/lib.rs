use luro_core::Command;
use crate::poise_commands::furaffinity;

mod functions;
mod structs;
pub mod poise_commands;

pub fn furaffinity_commands() -> [Command; 1] {
    [
        furaffinity(),
    ]
}