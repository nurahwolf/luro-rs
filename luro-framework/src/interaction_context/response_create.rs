use luro_builder::response::LuroResponse;
use luro_model::database::drivers::LuroDatabaseDriver;
use tracing::{error, warn};

use crate::{Framework, InteractionContext};
impl InteractionContext {
    /// Create a response. This is used for sending a response to an interaction, as well as to defer interactions.
    /// This CANNOT be used to update a response! Use `response_update` for that!
    pub async fn response_create<D: LuroDatabaseDriver>(
        &self,
        framework: Framework<D>,
        response: &LuroResponse
    ) -> anyhow::Result<()> {
        let client = framework.interaction_client(self.interaction.application_id);
        let request = response.interaction_response();
        if let Err(why) = client
            .create_response(self.interaction.id, &self.interaction.token, &request)
            .await
        {
            warn!(why = ?why, "Failed to send a response to an interaction, attempting to send as an update");
            if let Err(why) = self.response_update(framework, response).await {
                error!(why = ?why, "Failed to respond to interaction");
            }
        }
        Ok(())
    }
}
