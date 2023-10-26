use anyhow::anyhow;
use std::sync::Arc;

use crate::{LuroDatabase, LuroUser};

impl LuroUser {
    pub async fn update_permissions(&self, db: Arc<LuroDatabase>) -> anyhow::Result<u64> {
        match &self.data {
            Some(data) => Ok(db.update_user_permissions(self.user_id, &data.permissions).await?.rows_affected()),
            None => Err(anyhow!(
                "Could not update user permissions as user instance does not contain any luro data!"
            )),
        }
    }
}
