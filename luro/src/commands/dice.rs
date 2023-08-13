use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::LuroSlash;

mod help;
mod roll;
mod roll_direction;
mod simple;
mod stats;

use crate::luro_command::LuroCommand;

use self::help::DiceHelpCommand;
use self::roll::DiceRollCommand;
use self::roll_direction::DiceRollDirectionCommand;
use self::simple::DiceSimpleCommand;
use self::stats::DiceStatsCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "roll", desc = "Roll those freaking dice!!!")]
pub enum DiceCommands {
    #[command(name = "dice")]
    Roll(DiceRollCommand),
    #[command(name = "roll_direction")]
    RollDirection(DiceRollDirectionCommand),
    #[command(name = "stats")]
    Stats(DiceStatsCommand),
    #[command(name = "help")]
    Help(DiceHelpCommand),
    #[command(name = "simple")]
    Simple(DiceSimpleCommand)
}

impl LuroCommand for DiceCommands {
    async fn run_commands(self, ctx: LuroSlash) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Roll(command) => command.run_command(ctx).await,
            Self::RollDirection(command) => command.run_command(ctx).await,
            Self::Stats(command) => command.run_command(ctx).await,
            Self::Help(command) => command.run_command(ctx).await,
            Self::Simple(command) => command.run_command(ctx).await
        }
    }
}
