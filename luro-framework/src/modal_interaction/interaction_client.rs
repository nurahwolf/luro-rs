use twilight_http::client::InteractionClient;

use crate::ModalInteraction;

impl ModalInteraction {
    /// Create an interaction client
    pub fn interaction_client(&self) -> InteractionClient {
        self.twilight_client.interaction(self.application_id)
    }
}
