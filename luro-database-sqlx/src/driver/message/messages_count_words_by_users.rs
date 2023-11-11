use luro_model::types::MessageCount;

impl crate::SQLxDriver {
    pub async fn messages_count_words_by_users(&self) -> anyhow::Result<Vec<MessageCount>> {
        Ok(sqlx::query_file!("queries/messages_count_words_by_users.sql")
            .fetch_all(&self.pool)
            .await
            .map(|x| {
                x.into_iter().map(|count| MessageCount {
                    author_id: Some(twilight_model::id::Id::new(count.author_id as u64)),
                    total_messages: count.total_messages.unwrap_or_default(),
                    total_unique_words: count.total_unique_words.unwrap_or_default(),
                    total_words: count.total_words.unwrap_or_default(),
                }).collect()
            })?)
    }
}
