use luro_model::types::MessageCount;

impl crate::SQLxDriver {
    /// Returns total words, total unique words for all messages in the database
    pub async fn messages_count_words(&self) -> anyhow::Result<MessageCount> {
        Ok(sqlx::query_file!("queries/messages_count_words.sql")
            .fetch_one(&self.pool)
            .await
            .map(|total| MessageCount {
                author_id: None,
                total_messages: total.total_messages.unwrap_or_default(),
                total_unique_words: total.total_unique_words.unwrap_or_default(),
                total_words: total.total_words.unwrap_or_default(),
            })?)
    }
}
