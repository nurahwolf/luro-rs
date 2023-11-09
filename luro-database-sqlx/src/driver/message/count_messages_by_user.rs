use luro_model::types::UserWordCount;
use twilight_model::id::{marker::UserMarker, Id};

impl crate::SQLxDriver {
    pub async fn count_user_messages(&self, user_id: Id<UserMarker>) -> anyhow::Result<UserWordCount> {
        Ok(
            sqlx::query_file_as!(UserWordCount, "queries/messages/count_by_user.sql", user_id.get() as i64,)
                .fetch_one(&self.pool)
                .await?,
        )
    }
}
