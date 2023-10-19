use crate::LuroCharacter;

use super::LuroCharacterFetish;

impl LuroCharacter {
    pub async fn get_fetishes(&self) -> anyhow::Result<Vec<LuroCharacterFetish>> {
        let character = self.clone().into();
        Ok(self
            .db
            .get_character_fetishes(&character)
            .await?
            .into_iter()
            .map(|x| x.into())
            .collect::<Vec<_>>())
    }

    pub async fn get_fetish(&self, fetish_id: i64) -> anyhow::Result<Option<LuroCharacterFetish>> {
        let character = self.clone().into();
        Ok(self.db.get_character_fetish(&character, fetish_id).await?.map(|x| x.into()))
    }
}
