use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::LuroSlash;

mod help;
mod roll;
mod roll_direction;
mod stats;

use crate::traits::luro_command::LuroCommand;

use self::help::DiceHelpCommand;
use self::roll::DiceRollCommand;
use self::roll_direction::DiceRollDirectionCommand;
use self::stats::DiceStatsCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "dice", desc = "Roll those freaking dice!!!")]
pub enum DiceCommands {
    #[command(name = "roll")]
    Roll(DiceRollCommand),
    #[command(name = "roll_direction")]
    RollDirection(DiceRollDirectionCommand),
    #[command(name = "stats")]
    Stats(DiceStatsCommand),
    #[command(name = "help")]
    Help(DiceHelpCommand)
}

#[async_trait]
impl LuroCommand for DiceCommands {
    async fn run_commands(self, ctx: LuroSlash) -> anyhow::Result<()> {
        // Call the appropriate subcommand.
        match self {
            Self::Roll(command) => command.run_command(ctx).await,
            Self::RollDirection(command) => command.run_command(ctx).await,
            Self::Stats(command) => command.run_command(ctx).await,
            Self::Help(command) => command.run_command(ctx).await
        }
    }
}
