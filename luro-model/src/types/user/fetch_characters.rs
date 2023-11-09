use std::sync::Arc;

use crate::{LuroCharacter, LuroDatabase, LuroUser};

impl LuroUser {
    pub async fn fetch_character(&self, db: Arc<LuroDatabase>, name: &str) -> anyhow::Result<Option<LuroCharacter>> {
        Ok(db
            .get_user_character(self.user_id, name)
            .await?
            .map(|x| LuroCharacter::new(x, db.clone())))
    }
}
