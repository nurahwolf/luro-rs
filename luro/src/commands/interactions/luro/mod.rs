use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::models::interaction::{InteractionContext, InteractionResult};

mod nickname;

#[derive(CommandModel, CreateCommand)]
#[command(name = "luro", desc = "Do things to me! Oh my...")]
pub enum Command {
    #[command(name = "nickname")]
    Nickname(nickname::Command),
}

impl crate::models::CreateCommand for Command {
    async fn handle_command(self, ctx: &mut InteractionContext) -> InteractionResult<()> {
        match self {
            Self::Nickname(cmd) => cmd.handle_command(ctx).await,
        }
    }
}
