use luro_builder::response::LuroResponse;
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_http::Response;
use twilight_model::channel::Message;

use crate::{Framework, InteractionContext};

impl InteractionContext {
    /// Update an existing response
    pub async fn response_update<D: LuroDatabaseDriver>(
        &self,
        framework: Framework<D>,
        response: &LuroResponse
    ) -> anyhow::Result<Response<Message>> {
        let client = framework.interaction_client(self.interaction.application_id);
        let update_response = client
            .update_response(&self.interaction.token)
            .allowed_mentions(response.allowed_mentions.as_ref())
            .components(response.components.as_deref())
            .content(response.content.as_deref())
            .embeds(response.embeds.as_deref());

        Ok(update_response.await?)
    }
}
