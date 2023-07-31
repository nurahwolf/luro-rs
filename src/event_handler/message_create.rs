use std::sync::Arc;

use anyhow::Error;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::models::{LuroFramework, UserData};

impl LuroFramework {
    pub async fn message_create_listener(self: Arc<Self>, message: Box<MessageCreate>) -> Result<(), Error> {
        if !message.content.is_empty() {
            UserData::write_words(&self, &message.content, &message.author.id).await?;
        }

        Ok(())
    }
}
