use luro_model::{response::InteractionResponse, types::CommandResponse};
use tracing::warn;

use crate::ComponentInteraction;

impl ComponentInteraction {
    /// Create a response. This is used for sending a response to an interaction, as well as to defer interactions.
    /// This CANNOT be used to update a response! Use `response_update` for that!
    pub async fn response_create(&self, response: &InteractionResponse) -> anyhow::Result<CommandResponse> {
        let client = self.interaction_client();
        let request = response.interaction_response();

        match client.create_response(self.id, &self.interaction_token, &request).await {
            Ok(_) => Ok(CommandResponse::default()),
            Err(why) => {
                warn!(why = ?why, "Failed to send a response to an interaction, attempting to send as an update");
                self.response_update(response).await
            }
        }
    }
}
