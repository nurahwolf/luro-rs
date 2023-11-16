use luro_model::{response::InteractionResponse, types::CommandResponse};

use crate::ModalInteraction;

impl ModalInteraction {
    /// Update an existing response
    pub async fn response_update(&self, response: &InteractionResponse) -> anyhow::Result<CommandResponse> {
        let client = self.interaction_client();
        let request = client
            .update_response(&self.interaction_token)
            .allowed_mentions(response.allowed_mentions.as_ref())
            .components(response.components.as_deref())
            .content(response.content.as_deref())
            .embeds(response.embeds.as_deref());

        match response.attachments {
            Some(ref attachments) => Ok(request
                .attachments(attachments)
                .await?
                .model()
                .await
                .map(|x| CommandResponse { message: Some(x.into()) })?),
            None => Ok(request.await?.model().await.map(|x| CommandResponse { message: Some(x.into()) })?),
        }
    }
}
