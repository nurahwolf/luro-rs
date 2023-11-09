use std::sync::Arc;

use crate::{LuroCharacter, LuroDatabase, LuroUser};

impl LuroUser {
    pub async fn update_character(&self, db: Arc<LuroDatabase>, character: LuroCharacter) -> anyhow::Result<LuroCharacter> {
        Ok(db
            .update_user_character(character.into())
            .await
            .map(|x| LuroCharacter::new(x, db.clone()))?)
    }
}
