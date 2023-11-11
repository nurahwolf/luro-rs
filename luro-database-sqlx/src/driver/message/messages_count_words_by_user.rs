use luro_model::types::MessageCount;
use twilight_model::id::{marker::UserMarker, Id};

impl crate::SQLxDriver {
    pub async fn messages_count_words_by_user(&self, user_id: Id<UserMarker>) -> anyhow::Result<MessageCount> {
        Ok(sqlx::query_file!("queries/messages_count_words_by_user.sql", user_id.get() as i64,)
            .fetch_one(&self.pool)
            .await
            .map(|count| MessageCount {
                author_id: Some(Id::new(count.author_id as u64)),
                total_messages: count.total_messages.unwrap_or_default(),
                total_unique_words: count.total_unique_words.unwrap_or_default(),
                total_words: count.total_words.unwrap_or_default(),
            })?)
    }
}
