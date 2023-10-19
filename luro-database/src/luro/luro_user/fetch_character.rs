use std::collections::HashMap;

use crate::{LuroCharacter, LuroUser};

impl LuroUser {
    pub async fn fetch_characters(&self) -> anyhow::Result<HashMap<String, LuroCharacter>> {
        let mut map = HashMap::new();
        let characters = self.db.get_user_characters(self.user_id).await?;

        for character in characters {
            map.insert(character.character_name.clone(), LuroCharacter::new(character, self.db.clone()));
        }

        Ok(map)
    }
}
