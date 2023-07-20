use anyhow::Error;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::interaction::Interaction;

use crate::{interactions::InteractionResponse, LuroContext};

#[derive(CommandModel, CreateCommand)]
#[command(name = "hello", desc = "Say hello")]
pub struct HelloCommand {}

impl HelloCommand {
    pub async fn run(interaction: &Interaction, ctx: &LuroContext) -> Result<InteractionResponse, Error> {
        let ephemeral = ctx.defer_interaction(interaction, true).await?;

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
            ephemeral,
            deferred: true
        })
    }
}