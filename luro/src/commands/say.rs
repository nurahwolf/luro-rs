

use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

use crate::slash::Slash;

use crate::interaction::LuroSlash;
use crate::traits::luro_command::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "say", desc = "Make me say garbage!")]
pub struct SayCommand {
    /// The message to send.
    message: String,
    /// The user to send the message to.
    user: Option<ResolvedUser>
}


impl LuroCommand for SayCommand {
    async fn run_command(self, ctx: Slash) -> anyhow::Result<()> {
        let ctx = LuroSlash::new(ctx.framework, ctx.interaction);

        let content = if let Some(user) = self.user {
            format!("Hey <@{}>!\n{}", user.resolved.id, self.message)
        } else {
            self.message
        };

        ctx.respond(|response| response.content(content)).await
    }
}
