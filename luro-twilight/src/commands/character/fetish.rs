use luro_framework::{command::LuroCommandTrait, Framework, InteractionCommand};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand};

use self::add::Add;

mod add;

#[derive(CommandModel, CreateCommand)]
#[command(name = "fetish", desc = "Add or remove some fetishes to your profile")]
pub enum Fetish {
    #[command(name = "add")]
    Add(Add),
}
impl Fetish {
    pub async fn interaction_command<D: LuroDatabaseDriver>(
        self,
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        match self {
            Self::Add(_) => add::Add::handle_interaction(ctx, interaction).await,
        }
    }
}
