use std::sync::Arc;

use anyhow::Error;
use twilight_model::gateway::payload::incoming::MessageUpdate;

use crate::framework::LuroFramework;

impl LuroFramework {
    pub async fn message_update_handler(self: &Arc<Self>, message: MessageUpdate) -> Result<(), Error> {
        self.response_message_modified(&message.into()).await
    }
}
