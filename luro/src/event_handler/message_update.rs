use std::sync::Arc;

use anyhow::Error;
use twilight_model::gateway::payload::incoming::MessageUpdate;

use crate::framework::Framework;
use luro_model::luro_database_driver::LuroDatabaseDriver;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn message_update_handler(self: &Arc<Self>, message: MessageUpdate) -> Result<(), Error> {
        self.response_message_modified(&message.into()).await
    }
}
