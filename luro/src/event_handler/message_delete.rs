use std::sync::Arc;

use anyhow::Error;
use luro_model::database::drivers::LuroDatabaseDriver;
use twilight_model::gateway::payload::incoming::MessageDelete;

use crate::framework::Framework;
impl<D: LuroDatabaseDriver> Framework<D> {
    pub async fn message_delete_listener(self: &Arc<Self>, message: MessageDelete) -> Result<(), Error> {
        match self.twilight_cache.message(message.id) {
            Some(message) => self.response_message_modified(&message.clone().into()).await,
            None => Ok(())
        }
    }
}
