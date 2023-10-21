use crate::{LuroCharacter, LuroCharacterImage};

impl LuroCharacter {
    pub async fn update_image(&self, img: LuroCharacterImage) -> anyhow::Result<LuroCharacterImage> {
        Ok(self.db.update_character_image(img).await?)
    }
}
