impl crate::Database {
    pub async fn quote_fetch(&self, quote_id: i64) -> anyhow::Result<Option<luro_model::types::Message>> {
        let (message_id, channel_id) = match self.driver.quote_fetch(quote_id).await? {
            Some(quote_id) => quote_id,
            None => return Ok(None),
        };

        Ok(Some(self.message_fetch(message_id, Some(channel_id)).await?))
    }
}