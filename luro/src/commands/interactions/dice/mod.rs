use crate::models::interaction::{InteractionContext, InteractionResult};

mod help;
mod roll;
mod roll_direction;
mod simple;
mod stats;

#[derive(twilight_interactions::command::CommandModel, twilight_interactions::command::CreateCommand)]
#[command(name = "dice", desc = "Roll those freaking dice!!!")]
pub enum Dice {
    #[command(name = "direction")]
    Direction(roll_direction::Direction),
    #[command(name = "help")]
    Help(help::Help),
    #[command(name = "roll")]
    Roll(roll::Roll),
    #[command(name = "simple")]
    Simple(simple::Simple),
    #[command(name = "stats")]
    Stats(stats::Stats),
}

impl crate::models::CreateCommand for Dice {
    async fn handle_command(self, framework: &mut InteractionContext) -> InteractionResult<()> {
        match self {
            Self::Direction(cmd) => cmd.handle_command(framework).await,
            Self::Help(cmd) => cmd.handle_command(framework).await,
            Self::Roll(cmd) => cmd.handle_command(framework).await,
            Self::Simple(cmd) => cmd.handle_command(framework).await,
            Self::Stats(cmd) => cmd.handle_command(framework).await,
        }
    }
}
