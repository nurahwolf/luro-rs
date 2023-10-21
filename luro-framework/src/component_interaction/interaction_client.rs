use twilight_http::client::InteractionClient;

use crate::ComponentInteraction;

impl ComponentInteraction {
    /// Create an interaction client
    pub fn interaction_client(&self) -> InteractionClient {
        self.twilight_client.interaction(self.application_id)
    }
}
