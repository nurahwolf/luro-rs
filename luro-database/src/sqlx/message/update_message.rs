use luro_model::message::{LuroMessage, LuroMessageType};

use crate::LuroDatabase;

impl LuroDatabase {
    pub async fn update_message(&self, message: LuroMessageType) -> anyhow::Result<Option<LuroMessage>> {
        Ok(match message {
            #[cfg(feature = "cache")]
            LuroMessageType::CachedMessage(message) => self.handle_cached_message(message).await?,
            LuroMessageType::Custom(message) => self.handle_luro_message(message).await?,
            LuroMessageType::Message(message) => self.handle_message(message).await?,
            LuroMessageType::MessageCreate(message) => self.handle_message_create(message).await?,
            LuroMessageType::MessageDelete(message) => self.handle_message_delete(message).await?,
            LuroMessageType::MessageDeleteBulk(messages) => self.handle_message_delete_bulk(messages).await?,
            LuroMessageType::MessageUpdate(message) => self.handle_message_update(message).await?,
            LuroMessageType::None => None,
        })
    }
}
