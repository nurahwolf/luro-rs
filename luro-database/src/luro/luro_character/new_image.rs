use crate::{LuroCharacter, LuroCharacterImage};

impl LuroCharacter {
    pub async fn new_image(&self, img: LuroCharacterImage) -> anyhow::Result<LuroCharacterImage> {
        Ok(self.db.new_character_image(img).await?)
    }
}
