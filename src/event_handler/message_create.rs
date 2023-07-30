use std::sync::Arc;

use anyhow::Error;
use tracing::info;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::models::{LuroFramework, UserData};

impl LuroFramework {
    pub async fn message_create_listener(self: Arc<Self>, message: Box<MessageCreate>) -> Result<(), Error> {
        if !message.content.is_empty() {
            let lowercase = message.content.to_ascii_lowercase();
            let words: Vec<&str> = lowercase.split_whitespace().collect();
            UserData::write_words(&self, words, &message.author.id).await?;
        }

        if message.author.id != self.twilight_client.current_user().await?.model().await?.id
            || !message.author.bot
            || !message.content.is_empty()
        {
            info!("Message Received - Author: {}\n{}", message.author.name, message.0.content);
        }

        Ok(())
    }
}
