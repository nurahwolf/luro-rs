use std::sync::Arc;

use anyhow::Error;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::models::{LuroFramework, UserData};

impl LuroFramework {
    pub async fn message_create_listener(self: Arc<Self>, message: Box<MessageCreate>) -> Result<(), Error> {
        if !message.content.is_empty() {
            // TODO: Strip out special characters
            let lowercase = message.content.to_ascii_lowercase();
            let words: Vec<&str> = lowercase.split_whitespace().collect();
            UserData::write_words(&self, words, &message.author.id).await?;
        }

        Ok(())
    }
}
