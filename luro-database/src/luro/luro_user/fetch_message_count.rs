use std::sync::Arc;

use crate::{LuroDatabase, LuroUser, WordCount};

impl LuroUser {
    pub async fn fetch_message_count(&self, db: Arc<LuroDatabase>) -> anyhow::Result<WordCount> {
        Ok(db.count_user_messages(&self.user_id).await?)
    }
}
