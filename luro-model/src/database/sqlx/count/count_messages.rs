use crate::user::WordCount;

impl crate::database::sqlx::Database {
    pub async fn count_messages(&self) -> Result<WordCount, sqlx::Error> {
        sqlx::query_file_as!(WordCount, "queries/messages/count_all.sql",)
            .fetch_one(&self.pool)
            .await
    }
}
