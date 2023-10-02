use crate::sqlx::messages::anyhow;

use luro_model::message::{LuroMessage, LuroMessageSourceV2};

use crate::LuroDatabase;

impl LuroDatabase {
    pub async fn update_message(&self, message: LuroMessageSourceV2) -> anyhow::Result<Option<LuroMessage>> {
        Ok(match message {
            LuroMessageSourceV2::CachedMessage(_message) => todo!(),
            LuroMessageSourceV2::Custom(message) => self.handle_luro_message(message).await?,
            LuroMessageSourceV2::Message(_message) => todo!(),
            LuroMessageSourceV2::MessageCreate(message) => self.handle_message_create(message).await?,
            LuroMessageSourceV2::MessageDelete(message) => self.handle_message_delete(message).await?,
            LuroMessageSourceV2::MessageDeleteBulk(messages) => self.handle_message_delete_bulk(messages).await?,
            LuroMessageSourceV2::MessageUpdate(message) => self.handle_message_update(message).await?,
            LuroMessageSourceV2::None => return Err(anyhow!("No message data passed!")),
        })
    }
}
