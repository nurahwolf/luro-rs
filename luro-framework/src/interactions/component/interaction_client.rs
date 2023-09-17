use twilight_http::client::InteractionClient;

use crate::ComponentInteraction;

impl<T> ComponentInteraction<T> {
    /// Create an interaction client
    pub fn interaction_client(&self) -> InteractionClient {
        self.twilight_client.interaction(self.application_id)
    }
}
