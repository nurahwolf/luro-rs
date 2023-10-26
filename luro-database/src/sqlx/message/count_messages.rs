use crate::WordCount;

impl crate::LuroDatabase {
    pub async fn count_messages(&self) -> Result<WordCount, sqlx::Error> {
        sqlx::query_file_as!(WordCount, "queries/messages/count_all.sql",)
            .fetch_one(&self.pool)
            .await
    }
}
