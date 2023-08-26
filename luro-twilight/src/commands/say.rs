use luro_framework::{command::LuroCommand, Framework, InteractionCommand, LuroInteraction};
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand)]
#[command(name = "say", desc = "Make me say garbage!")]
pub struct Say {
    /// The message to send.
    message: String,
    /// The user to send the message to.
    user: Option<ResolvedUser>
}

impl LuroCommand for Say {
    async fn interaction_command<D: LuroDatabaseDriver>(
        self,
        ctx: Framework<D>,
        interaction: InteractionCommand
    ) -> anyhow::Result<()> {
        let content = if let Some(user) = self.user {
            format!("Hey <@{}>!\n{}", user.resolved.id, self.message)
        } else {
            self.message
        };

        interaction.respond(&ctx, |response| response.content(content)).await?;
        Ok(())
    }
}
