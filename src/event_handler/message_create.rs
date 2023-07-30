use std::sync::Arc;

use anyhow::Error;
use tracing::info;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::models::{LuroFramework, UserData};

impl LuroFramework {
    pub async fn message_create_listener(self: Arc<Self>, message: Box<MessageCreate>) -> Result<(), Error> {
        if message.content.to_ascii_lowercase().contains("bah") {
            UserData::write_words(&self, "bah", &message.author.id).await?;
        }

        if message.content.to_ascii_lowercase().contains("owo") {
            UserData::write_words(&self, "owo", &message.author.id).await?;
        }

        if message.content.to_ascii_lowercase().contains("uwu") {
            UserData::write_words(&self, "uwu", &message.author.id).await?;
        }

        if message.content.to_ascii_lowercase().contains("heck") {
            UserData::write_words(&self, "heck", &message.author.id).await?;
        }

        if message.content.to_ascii_lowercase().contains("hmm") {
            UserData::write_words(&self, "hmm", &message.author.id).await?;
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
