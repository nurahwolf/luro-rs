use std::sync::Arc;

use anyhow::Error;
use twilight_model::gateway::payload::incoming::MessageDelete;

use crate::framework::LuroFramework;

impl LuroFramework {
    pub async fn message_delete_listener(self: &Arc<Self>, message: MessageDelete) -> Result<(), Error> {
        self.response_message_modified(&message.into()).await
    }
}
