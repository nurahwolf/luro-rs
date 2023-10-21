use std::sync::Arc;

use crate::{LuroCharacter, LuroDatabase, LuroUser};

impl LuroUser {
    pub async fn update_character_prefix(&self, character: LuroCharacter, db: Arc<LuroDatabase>) -> anyhow::Result<LuroCharacter> {
        Ok(db
            .update_user_character_prefix(character.into())
            .await
            .map(|x| LuroCharacter::new(x, db.clone()))?)
    }
}
