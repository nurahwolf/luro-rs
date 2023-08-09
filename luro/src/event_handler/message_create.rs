use std::sync::Arc;

use anyhow::Error;
use luro_model::luro_database_driver::LuroDatabaseDriver;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::framework::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn message_create_listener(self: &Arc<Self>, message: MessageCreate) -> Result<(), Error> {
        self.response_message_modified(&message.into()).await
    }
}
