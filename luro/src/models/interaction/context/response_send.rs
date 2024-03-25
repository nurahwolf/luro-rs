use luro_model::builders::InteractionResponseBuilder;
use twilight_model::http::interaction::InteractionResponseType;

use crate::models::interaction::InteractionResult;

impl super::InteractionContext {
    /// Send an existing response against an interaction.
    pub async fn response_send(&self, response: &InteractionResponseBuilder) -> InteractionResult<()> {
        let client = self.interaction_client();
        let request = response.interaction_response();

        if response.interaction_response_type == InteractionResponseType::DeferredChannelMessageWithSource
            || response.interaction_response_type == InteractionResponseType::DeferredUpdateMessage
        {
            let mut client = client.update_response(&self.interaction.token);

            if let Some(attachments) = &response.attachments {
                client = client.attachments(attachments)
            }

            client
                .components(response.components.as_deref())
                .content(response.content.as_deref())
                .embeds(response.embeds.as_deref())
                .await?;
            return Ok(());
        }

        if let Err(why) = client.create_response(self.interaction.id, &self.interaction.token, &request).await {
            tracing::error!(?why, "Failed to send a new interaction response!")
        }

        Ok(())
    }
}
