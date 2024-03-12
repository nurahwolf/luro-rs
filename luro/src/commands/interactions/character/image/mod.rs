use luro_framework::{CommandInteraction, LuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand};

// mod add;
mod get;

#[derive(CommandModel, CreateCommand)]
#[command(name = "img", desc = "Get or modify your character's images")]
pub enum Image {
    // #[command(name = "add")]
    // Add(add::Add),
    #[command(name = "get")]
    Get(get::Get),
}
impl LuroCommand for Image {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<luro_model::types::CommandResponse> {
        match self {
            // Self::Add(command) => command.interaction_command(ctx).await,
            Self::Get(command) => command.interaction_command(ctx).await,
        }
    }
}
