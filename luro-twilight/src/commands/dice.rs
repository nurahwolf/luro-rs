use luro_framework::command::LuroCommand;
use luro_framework::{Framework, InteractionCommand};
use luro_model::database::drivers::LuroDatabaseDriver;
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

#[derive(CommandModel, CreateCommand)]
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
    Simple(Simple)
}

impl LuroCommand for Dice {
    async fn interaction_command<D: LuroDatabaseDriver>(
        self,
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        match self {
            Self::Roll(command) => command.interaction_command(ctx, interaction).await,
            Self::Direction(command) => command.interaction_command(ctx, interaction).await,
            Self::Stats(command) => command.interaction_command(ctx, interaction).await,
            Self::Help(command) => command.interaction_command(ctx, interaction).await,
            Self::Simple(command) => command.interaction_command(ctx, interaction).await
        }
    }
}
