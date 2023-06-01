use anyhow::Error;
use tracing::info;
use twilight_model::gateway::payload::incoming::MessageUpdate;

pub async fn message_update_handler(message: Box<MessageUpdate>) -> Result<(), Error> {
    let content = match message.content {
        Some(content) => content,
        None => "No Content Available".to_string(),
    };

    info!("Message Updated - Author: TODO\nContent: {}", content);

    Ok(())
}
