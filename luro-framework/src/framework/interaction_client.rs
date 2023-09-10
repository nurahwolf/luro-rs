use luro_model::database_driver::LuroDatabaseDriver;
use twilight_http::client::InteractionClient;
use twilight_model::id::{marker::ApplicationMarker, Id};

use crate::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    /// Create an interaction client
    pub async fn new_interaction_client(&self) -> anyhow::Result<InteractionClient> {
        let application = self.twilight_client.current_user_application().await?.model().await?;
        Ok(self.twilight_client.interaction(application.id))
    }

    /// Create an interaction client
    pub fn interaction_client(&self, application_id: Id<ApplicationMarker>) -> InteractionClient {
        self.twilight_client.interaction(application_id)
    }
}
