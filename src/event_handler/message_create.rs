use anyhow::Error;
use tracing::info;
use twilight_model::{gateway::payload::incoming::MessageCreate, id::Id};

pub async fn message_create_listener(message: Box<MessageCreate>) -> Result<(), Error> {
    if message.author.id != Id::new(180285980232646656)
        || !message.author.bot
        || !message.content.is_empty()
    {
        info!(
            "Message Received - Author: {}\nContent: {}",
            message.author.name, message.0.content
        );
    }

    Ok(())
}
