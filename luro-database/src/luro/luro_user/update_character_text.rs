use crate::{LuroCharacter, LuroUser};

impl LuroUser {
    pub async fn update_character_text(&self, character: LuroCharacter) -> anyhow::Result<LuroCharacter> {
        Ok(self.db.update_user_character_text(character.into()).await.map(|x|LuroCharacter::new(x, self.db.clone()))?)
    }
}
