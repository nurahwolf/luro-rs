use std::sync::Arc;

use crate::{DbUserMarriage, LuroDatabase, LuroUser};

impl LuroUser {
    pub async fn fetch_marriages(&self, db: Arc<LuroDatabase>) -> anyhow::Result<Vec<DbUserMarriage>> {
        Ok(db.get_marriages(self.user_id).await?)
    }
}
