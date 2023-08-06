use twilight_http::client::InteractionClient;

use crate::models::LuroResponse;

use super::LuroFramework;

impl LuroFramework {
    /// Create an interaction client
    pub fn interaction_client(&self, slash: &LuroResponse) -> InteractionClient {
        self.twilight_client.interaction(slash.interaction.application_id)
    }
}
