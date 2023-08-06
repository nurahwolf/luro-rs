use async_trait::async_trait;

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use crate::models::LuroSlash;

use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "say", desc = "Make me say garbage!")]
pub struct SayCommand {
    /// The message to send.
    message: String,
    /// The user to send the message to.
    user: Option<ResolvedUser>
}

#[async_trait]
impl LuroCommand for SayCommand {
    async fn run_command(self, mut ctx: LuroSlash) -> anyhow::Result<()> {
        let content = if let Some(user) = self.user {
            format!("Hey <@{}>!\n{}", user.resolved.id, self.message)
        } else {
            self.message
        };

        ctx.content(content).respond().await
    }
}
