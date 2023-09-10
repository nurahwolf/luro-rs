use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::LuroSlash;
use luro_model::database_driver::LuroDatabaseDriver;

mod help;
mod roll;
mod roll_direction;
mod simple;
mod stats;

use crate::luro_command::LuroCommand;

use self::help::Help;
use self::roll::Roll;
use self::roll_direction::Direction;
use self::simple::Simple;
use self::stats::Stats;
#[derive(CommandModel, CreateCommand)]
#[command(name = "dice", desc = "Roll those freaking dice!!!")]
pub enum DiceCommands {
    #[command(name = "roll")]
    Roll(Roll),
    #[command(name = "direction")]
    Direction(Direction),
    #[command(name = "stats")]
    Stats(Stats),
    #[command(name = "help")]
    Help(Help),
    #[command(name = "simple")]
    Simple(Simple),
}

impl LuroCommand for DiceCommands {
    async fn run_command<D: LuroDatabaseDriver>(self, ctx: LuroSlash<D>) -> anyhow::Result<()> {
        match self {
            Self::Roll(command) => command.run_command(ctx).await,
            Self::Direction(command) => command.run_command(ctx).await,
            Self::Stats(command) => command.run_command(ctx).await,
            Self::Help(command) => command.run_command(ctx).await,
            Self::Simple(command) => command.run_command(ctx).await,
        }
    }
}
