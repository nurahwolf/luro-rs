use twilight_http::{Error, Response};
use twilight_model::{application::interaction::Interaction, channel::Message};

use crate::{database::drivers::LuroDatabaseDriver, response::LuroResponse};

use super::Context;

impl<D: LuroDatabaseDriver> Context<D> {
    /// Update an existing response
    pub async fn update_response(&self, response: &LuroResponse, interaction: &Interaction) -> Result<Response<Message>, Error> {
        self.interaction_client(interaction.application_id)
            .update_response(&interaction.token)
            .allowed_mentions(response.allowed_mentions.as_ref())
            .components(response.components.as_deref())
            .content(response.content.as_deref())
            .embeds(response.embeds.as_deref())
            .await
    }
}
