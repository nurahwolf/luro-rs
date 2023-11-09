use luro_model::types::UserData;

use crate::Database;

impl Database {
    pub async fn user_update_permissions(&self, user_data: &UserData) -> anyhow::Result<u64> {
        // TODO: Can I remove this clone?
        Ok(self.driver.update_user_permissions(user_data.user_id, user_data.permissions.clone()).await?.rows_affected())
    }
}
