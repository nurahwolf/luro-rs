use async_trait::async_trait;
use luro_framework::{
    command::{LuroCommandBuilder, LuroCommandTrait},
    Framework, InteractionCommand, LuroInteraction,
};
use luro_model::database_driver::LuroDatabaseDriver;
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand)]
#[command(name = "say", desc = "Make me say garbage!")]
pub struct Say {
    /// The message to send.
    message: String,
    /// The user to send the message to.
    user: Option<ResolvedUser>,
}

impl<D: LuroDatabaseDriver + 'static> LuroCommandBuilder<D> for Say {}

#[async_trait]
impl LuroCommandTrait for Say {
    async fn handle_interaction<D: LuroDatabaseDriver>(
        ctx: Framework<D>,
        interaction: InteractionCommand,
    ) -> anyhow::Result<()> {
        let data = Self::new(interaction.data.clone())?;
        let content = if let Some(user) = data.user {
            format!("Hey <@{}>!\n{}", user.resolved.id, data.message)
        } else {
            data.message
        };

        interaction.respond(&ctx, |response| response.content(content)).await?;
        Ok(())
    }
}
