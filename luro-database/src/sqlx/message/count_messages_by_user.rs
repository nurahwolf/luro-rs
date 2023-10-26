use twilight_model::id::{marker::UserMarker, Id};

use crate::{LuroDatabase, WordCount};

impl LuroDatabase {
    pub async fn count_user_messages(&self, user_id: &Id<UserMarker>) -> Result<WordCount, sqlx::Error> {
        sqlx::query_file_as!(WordCount, "queries/messages/count_by_user.sql", user_id.get() as i64,)
            .fetch_one(&self.pool)
            .await
    }
}
