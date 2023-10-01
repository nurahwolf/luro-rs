use luro_model::message::LuroMessage;
use sqlx::Error;
use twilight_model::gateway::payload::incoming::{MessageDeleteBulk, MessageDelete};

use crate::LuroDatabase;

impl LuroDatabase {
    pub async fn handle_message_delete_bulk(&self, messages: MessageDeleteBulk) -> Result<Option<LuroMessage>, Error> {
        let mut final_message = None;
        for message in messages.ids {
            final_message = self.handle_message_delete(MessageDelete {
                channel_id: messages.channel_id,
                guild_id: messages.guild_id,
                id: message,
            }).await?
        }

        Ok(final_message)
        
    }
}