use async_trait::async_trait;
use luro_framework::{
    command::LuroCommandTrait, LuroInteraction, CommandInteraction,
};
use twilight_interactions::command::{CommandModel, CreateCommand, ResolvedUser};

#[derive(CommandModel, CreateCommand)]
#[command(name = "say", desc = "Make me say garbage!")]
pub struct Say {
    /// The message to send.
    message: String,
    /// The user to send the message to.
    user: Option<ResolvedUser>,
}

#[async_trait]
impl LuroCommandTrait for Say {
    async fn handle_interaction(
        ctx: CommandInteraction<Self>,
    ) -> anyhow::Result<()> {
        let content = if let Some(user) = ctx.command.user {
            format!("Hey <@{}>!\n{}", user.resolved.id, ctx.command.message)
        } else {
            ctx.command.message
        };

        content.respond(&ctx, |response| response.content(content)).await?;
        Ok(())
    }
}
