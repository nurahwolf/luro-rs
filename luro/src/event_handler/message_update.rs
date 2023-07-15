use anyhow::Error;
use tracing::info;
use twilight_model::gateway::payload::incoming::MessageUpdate;

use crate::framework::LuroFramework;

impl LuroFramework {
    pub async fn message_update_handler(message: Box<MessageUpdate>) -> Result<(), Error> {
        if let Some(content) = message.content && let Some(author) = message.author {
            info!("Message Updated - Author: {}\nContent: {}", author.name, content);
        };
    
        Ok(())
    }
}

