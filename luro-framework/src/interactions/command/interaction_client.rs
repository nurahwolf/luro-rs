use twilight_http::client::InteractionClient;

use crate::CommandInteraction;

impl<T> CommandInteraction<T> {
    /// Create an interaction client
    pub fn interaction_client(&self) -> InteractionClient {
        self.twilight_client.interaction(self.application_id)
    }
}
