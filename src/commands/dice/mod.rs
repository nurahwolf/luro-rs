use crate::Command;

mod command_roll;
mod command_roll_dice;
mod command_roll_direction;
mod command_roll_help;
mod command_roll_stats;
mod function_diceroller;

pub fn commands() -> [Command; 1] {
    [command_roll::roll()]
}
