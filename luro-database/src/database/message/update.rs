use luro_model::sync::MessageSync;

impl crate::Database {
    pub async fn message_update(&self, message: impl Into<MessageSync<'_>>) -> anyhow::Result<u64> {
        self.driver.update_message(message).await
    }
}
