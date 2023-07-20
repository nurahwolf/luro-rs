use anyhow::Error;
use tracing::info;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::framework::LuroFramework;

impl LuroFramework {
    pub async fn message_create_listener(&self, message: Box<MessageCreate>) -> Result<(), Error> {
        if message.author.id != self.twilight_client.current_user().await?.model().await?.id
            || !message.author.bot
            || !message.content.is_empty()
        {
            info!(
                "Message Received - Author: {}\n{}",
                message.author.name, message.0.content
            );
        }

        Ok(())
    }
}
