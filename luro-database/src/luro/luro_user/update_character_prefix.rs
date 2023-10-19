use crate::{LuroCharacter, LuroUser};

impl LuroUser {
    pub async fn update_character_prefix(&self, character: LuroCharacter) -> anyhow::Result<LuroCharacter> {
        Ok(self.db.update_user_character_prefix(character.into()).await.map(|x|LuroCharacter::new(x, self.db.clone()))?)
    }
}
