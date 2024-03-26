use twilight_model::id::{marker::UserMarker, Id};

use crate::user::WordCount;

impl crate::database::sqlx::Database {
    pub async fn count_messages_user(&self, user_id: Id<UserMarker>) -> Result<WordCount, sqlx::Error> {
        Ok(
            sqlx::query_file_as!(WordCount, "queries/messages/count_by_user.sql", user_id.get() as i64,)
                .fetch_one(&self.pool)
                .await?,
        )
    }
}
