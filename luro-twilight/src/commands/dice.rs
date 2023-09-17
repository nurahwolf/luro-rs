use async_trait::async_trait;
use luro_framework::command::ExecuteLuroCommand;
use luro_framework::CommandInteraction;
use twilight_interactions::command::{CommandModel, CreateCommand};

mod help;
mod roll;
mod roll_direction;
mod simple;
mod stats;

use self::help::Help;
use self::roll::Roll;
use self::roll_direction::Direction;
use self::simple::Simple;
use self::stats::Stats;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "dice", desc = "Roll those freaking dice!!!")]
pub enum Dice {
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

#[async_trait]
impl ExecuteLuroCommand for Dice {
    async fn interaction_command(&self, ctx: CommandInteraction<()>) -> anyhow::Result<()> {
        match self {
            Self::Roll(command) => command.interaction_command(ctx).await,
            Self::Direction(command) => command.interaction_command(ctx).await,
            Self::Stats(command) => command.interaction_command(ctx).await,
            Self::Help(command) => command.interaction_command(ctx).await,
            Self::Simple(command) => command.interaction_command(ctx).await,
        }
    }
}
