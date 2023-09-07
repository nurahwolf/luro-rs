use async_trait::async_trait;
use luro_framework::command::{LuroCommandBuilder, LuroCommandTrait};
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

impl<D: LuroDatabaseDriver + 'static> LuroCommandBuilder<D> for Dice {}

#[async_trait]
impl LuroCommandTrait for Dice {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        match data {
            Self::Roll(_command) => roll::Roll::handle_interaction(ctx, interaction).await,
            Self::Direction(_command) => roll_direction::Direction::handle_interaction(ctx, interaction).await,
            Self::Stats(_command) => stats::Stats::handle_interaction(ctx, interaction).await,
            Self::Help(_command) => help::Help::handle_interaction(ctx, interaction).await,
            Self::Simple(_command) => simple::Simple::handle_interaction(ctx, interaction).await
        }
    }
}
