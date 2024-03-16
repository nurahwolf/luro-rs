use crate::{builders::InteractionResponseBuilder, models::interaction::InteractionResult};

impl super::InteractionContext {
    /// Send an existing response against an interaction.
    pub async fn response_update(&self, response: &InteractionResponseBuilder) -> InteractionResult<()> {
        let client = self.interaction_client();

        let mut client = client.update_response(&self.interaction.token);

        if let Some(attachments) = &response.attachments {
            client = client.attachments(attachments)
        }

        client
            .components(response.components.as_deref())
            .content(response.content.as_deref())
            .embeds(response.embeds.as_deref())
            .await?;

        Ok(())
    }
}
