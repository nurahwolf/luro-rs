use luro_model::types::UserWordCount;

impl crate::SQLxDriver {
    pub async fn count_messages(&self) -> Result<UserWordCount, sqlx::Error> {
        sqlx::query_file_as!(UserWordCount, "queries/messages/count_all.sql",)
            .fetch_one(&self.pool)
            .await
    }
}
