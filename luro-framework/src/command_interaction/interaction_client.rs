use twilight_http::client::InteractionClient;

use crate::CommandInteraction;

impl CommandInteraction {
    /// Create an interaction client
    pub fn interaction_client(&self) -> InteractionClient {
        self.twilight_client.interaction(self.application_id)
    }
}
