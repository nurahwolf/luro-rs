use luro_model::response::LuroResponse;
use twilight_model::channel::Message;

use crate::ModalInteraction;

impl ModalInteraction {
    /// Update an existing response
    pub async fn response_update(&self, response: &LuroResponse) -> anyhow::Result<Message> {
        Ok(self
            .interaction_client()
            .update_response(&self.interaction_token)
            .allowed_mentions(response.allowed_mentions.as_ref())
            .components(response.components.as_deref())
            .content(response.content.as_deref())
            .embeds(response.embeds.as_deref())
            .await?
            .model()
            .await?)
    }
}
