use crate::{LuroCharacter, LuroCharacterImage};

impl LuroCharacter {
    pub async fn fetch_images(&self) -> anyhow::Result<Vec<LuroCharacterImage>> {
        let character = self.clone().into();
        Ok(self.db.get_character_images(&character).await?)
    }

    pub async fn fetch_image(&self, img_id: i64) -> anyhow::Result<Option<LuroCharacterImage>> {
        let character = self.clone().into();
        Ok(self.db.get_character_image(&character, img_id).await?)
    }
}
