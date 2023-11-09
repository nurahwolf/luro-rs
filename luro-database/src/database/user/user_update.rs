use luro_model::sync::UserSync;

use crate::Database;

impl Database {
    pub async fn user_update(&self, user: impl Into<UserSync>) -> anyhow::Result<u64> {
        self.driver.update_user(user).await
    }
}