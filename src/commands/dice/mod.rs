use crate::Command;

mod roll;
mod roll_dice;
mod roll_direction;
mod roll_help;
mod roll_stats;

pub fn commands() -> [Command; 1] {
    [roll::roll()]
}
