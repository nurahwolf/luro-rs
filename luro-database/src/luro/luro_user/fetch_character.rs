use std::{collections::HashMap, sync::Arc};

use crate::{LuroCharacter, LuroDatabase, LuroUser};

impl LuroUser {
    pub async fn fetch_characters(&self, db: Arc<LuroDatabase>) -> anyhow::Result<HashMap<String, LuroCharacter>> {
        let mut map = HashMap::new();
        let characters = db.get_user_characters(self.user_id).await?;

        for character in characters {
            map.insert(character.character_name.clone(), LuroCharacter::new(character, db.clone()));
        }

        Ok(map)
    }
}
