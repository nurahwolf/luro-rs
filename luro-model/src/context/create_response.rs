use twilight_http::{response::marker::EmptyBody, Error, Response};
use twilight_model::application::interaction::Interaction;

use crate::{database::drivers::LuroDatabaseDriver, response::LuroResponse};

use super::Context;

impl<D: LuroDatabaseDriver> Context<D> {
    /// Create a response. This is used for sending a response to an interaction, as well as to defer interactions.
    pub async fn create_response(
        &self,
        response: &LuroResponse,
        interaction: &Interaction
    ) -> Result<Response<EmptyBody>, Error> {
        let request = response.interaction_response();
        self.interaction_client(interaction.application_id)
            .create_response(interaction.id, &interaction.token, &request)
            .await
    }
}
