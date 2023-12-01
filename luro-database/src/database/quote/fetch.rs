impl crate::Database {
    pub async fn quote_fetch(&self, quote_id: i64) -> anyhow::Result<Option<luro_model::types::Quote>> {
        self.driver.quote_fetch(quote_id).await
    }
}