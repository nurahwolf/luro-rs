use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::interaction::{InteractionContext, InteractionResult};

mod create;

#[derive(CommandModel, CreateCommand, Debug, PartialEq, Eq)]
#[command(name = "fetish", desc = "Add or remove some fetishes to your profile")]
pub enum Command {
    #[command(name = "create")]
    Create(create::Command),
}
impl crate::models::CreateCommand for Command {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        match self {
            Self::Create(cmd) => cmd.handle_command(ctx).await,
        }
    }
}
