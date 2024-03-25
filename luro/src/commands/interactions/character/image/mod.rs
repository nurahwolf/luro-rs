use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::interaction::{InteractionContext, InteractionResult};

mod add;
mod get;

#[derive(CommandModel, CreateCommand)]
#[command(name = "img", desc = "Get or modify your character's images")]
pub enum Command {
    #[command(name = "add")]
    Add(add::Command),
    #[command(name = "get")]
    Get(get::Command),
}
impl crate::models::CreateCommand for Command {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        match self {
            // Self::Add(cmd) => command.interaction_command(ctx).await,
            Self::Get(cmd) => command.interaction_command(ctx).await,
        }
    }
}
