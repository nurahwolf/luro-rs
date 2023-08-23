use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_http::client::InteractionClient;
use twilight_model::id::{marker::ApplicationMarker, Id};

use crate::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    /// Create an interaction client
    pub fn interaction_client(&self, application_id: Id<ApplicationMarker>) -> InteractionClient {
        self.twilight_client.interaction(application_id)
    }
}
