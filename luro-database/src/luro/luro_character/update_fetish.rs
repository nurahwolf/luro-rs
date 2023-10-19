use crate::LuroCharacter;

use super::LuroCharacterFetish;

impl LuroCharacter {
    pub async fn update_fetish(&self, fetish: LuroCharacterFetish) -> anyhow::Result<LuroCharacterFetish> {
        Ok(self.db.update_character_fetish(fetish.into()).await.map(|x| x.into())?)
    }
}
