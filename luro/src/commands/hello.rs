use async_trait::async_trait;
use twilight_gateway::MessageSender;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;

use crate::{interactions::InteractionResponse, LuroContext, SlashResponse};

use super::LuroCommand;
#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Say hello")]
pub struct HelloCommand {}

#[async_trait]
impl LuroCommand for HelloCommand {
    async fn run_command(self, interaction: Interaction, ctx: LuroContext, _shard: MessageSender) -> SlashResponse {
        let luro_response = ctx.defer_interaction(&interaction, false).await?;

        let message = match interaction.author_id() {
            Some(author_id) => format!(
                "Hello World! I am **{}**. It's nice to meet you, <@{}>!",
                ctx.twilight_client.current_user().await?.model().await?.name,
                author_id
            ),
            None => format!(
                "Hello World! I am **{}**. It's nice to meet you, but unfortunately I cannot see your name :(",
                ctx.twilight_client.current_user().await?.model().await?.name
            )
        };

        Ok(InteractionResponse::Content {
            content: message,
            luro_response
        })
    }
}
