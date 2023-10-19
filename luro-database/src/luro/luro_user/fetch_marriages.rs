use crate::{LuroUser, DbUserMarriage};

impl LuroUser {
    pub async fn fetch_marriages(&self) -> anyhow::Result<Vec<DbUserMarriage>> {
        Ok(self.db.get_marriages(self.user_id).await?)
    }
}
