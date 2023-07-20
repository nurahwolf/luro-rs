use anyhow::Error;

use tracing::info;
use twilight_model::gateway::payload::incoming::MessageDelete;

use crate::LuroFramework;
impl LuroFramework {
    pub async fn message_delete_listener(&self, message: MessageDelete) -> Result<(), Error> {
        if let Some(message) = self.twilight_cache.message(message.id) {
            let author = message.author();
            let content = message.content();

            info!("Message Deleted - Author ID: {}\nContent: {}", author, content);
        };

        Ok(())
    }
}
