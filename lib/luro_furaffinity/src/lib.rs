use crate::poise_commands::furaffinity;
use luro_core::Command;

mod functions;
pub mod poise_commands;
mod structs;

pub fn furaffinity_commands() -> [Command; 1] {
    [furaffinity()]
}
