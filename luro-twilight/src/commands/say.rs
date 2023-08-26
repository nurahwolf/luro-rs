use anyhow::anyhow;
use luro_framework::{Framework, InteractionCommand};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_model::application::interaction::application_command::CommandData;
use luro_framework::interaction_context::LuroInteraction;

#[derive(CommandModel, CreateCommand)]
#[command(name = "say", desc = "Make me say garbage!")]
pub struct Say {
    /// The message to send.
    message: String,
    /// The user to send the message to.
    user: Option<ResolvedUser>
}

impl Say {
    pub fn new(data: Box<CommandData>) -> anyhow::Result<Self> {
        let data = *data;
        match Self::from_interaction(data.into()) {
            Ok(ok) => Ok(ok),
            Err(why) => Err(anyhow!(
                "Got interaction data, but failed to parse it to the command type specified: {why}"
            ))
        }
    }

    pub async fn run_command<D: LuroDatabaseDriver>(self, ctx: Framework<D>, interaction: InteractionCommand) -> anyhow::Result<()> {
        let content = if let Some(user) = self.user {
            format!("Hey <@{}>!\n{}", user.resolved.id, self.message)
        } else {
            self.message
        };

        interaction.respond(&ctx, |response| response.content(content)).await?;
        Ok(())
    }
}
