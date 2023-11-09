use luro_model::types::UserWordCount;
use twilight_model::id::{Id, marker::UserMarker};

impl crate::Database {
    pub async fn user_count_messages(&self, user_id: Id<UserMarker>) -> anyhow::Result<UserWordCount> {
        self.driver.count_user_messages(user_id).await
    }
}