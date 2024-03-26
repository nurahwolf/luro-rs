use twilight_model::id::{marker::UserMarker, Id};

use crate::user::WordCount;

impl crate::database::Database {
    /// Return how many messages are cached by the client / in the database.
    pub async fn count_messages_user(&self, user_id: Id<UserMarker>) -> WordCount {
        let mut total_messages = WordCount::default();

        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.count_messages_user(user_id).await {
            Ok(messages) => total_messages = messages,
            Err(why) => tracing::error!(?why, "COUNT_MESSAGES_USER: error checking the database"),
        }

        total_messages
    }
}
