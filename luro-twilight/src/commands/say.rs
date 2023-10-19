use luro_framework::{CommandInteraction, CreateLuroCommand};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand, Debug)]
#[command(name = "say", desc = "Make me say garbage!")]
pub struct Say {
    /// The message to send.
    message: String,
    /// The user to send the message to.
    user: Option<ResolvedUser>,
}

impl CreateLuroCommand for Say {
    async fn interaction_command(self, ctx: CommandInteraction) -> anyhow::Result<()> {
        let content = if let Some(ref user) = self.user {
            format!("Hey <@{}>!\n{}", user.resolved.id, self.message)
        } else {
            self.message.clone()
        };

        ctx.respond(|response| response.content(content)).await?;
        Ok(())
    }
}
