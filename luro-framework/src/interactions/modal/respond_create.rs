use luro_model::response::LuroResponse;
use tracing::warn;
use twilight_model::channel::Message;

use crate::ModalInteraction;

impl<T> ModalInteraction<T> {    /// Create a response. This is used for sending a response to an interaction, as well as to defer interactions.
    /// This CANNOT be used to update a response! Use `response_update` for that!
    pub async fn response_create(
        &self,
        response: &LuroResponse,
    ) -> anyhow::Result<Option<Message>> {
        let client = self.interaction_client();
        let request = response.interaction_response();

        match client.create_response(self.id, &self.token, &request).await {
            Ok(_) => Ok(None),
            Err(why) => {
                warn!(why = ?why, "Failed to send a response to an interaction, attempting to send as an update");
                Ok(Some(self.response_update(response).await?))
            }
        }
    }
}