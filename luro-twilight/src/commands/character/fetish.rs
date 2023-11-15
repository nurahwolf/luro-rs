use luro_framework::{CommandInteraction, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

mod create;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "fetish", desc = "Add or remove some fetishes to your profile")]
pub enum Fetish {
    #[command(name = "create")]
    Create(create::Create),
}
impl LuroCommand for Fetish {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        match self {
            Self::Create(command) => command.interaction_command(ctx).await,
        }
    }
}
