use twilight_http::client::InteractionClient;

use crate::Framework;

impl Framework {
    /// Create an interaction client
    pub async fn interaction_client(&self) -> anyhow::Result<InteractionClient> {
        Ok(self
            .twilight_client
            .interaction(self.twilight_client.current_user_application().await?.model().await?.id))
    }
}
