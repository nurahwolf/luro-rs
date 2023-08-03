use std::sync::Arc;

use anyhow::Error;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::models::LuroFramework;

impl LuroFramework {
    pub async fn message_create_listener(self: &Arc<Self>, message: MessageCreate) -> Result<(), Error> {
        self.response_message_modified(&message.into()).await
    }
}
