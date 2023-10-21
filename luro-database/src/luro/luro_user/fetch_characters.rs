use crate::{LuroCharacter, LuroUser};

impl LuroUser {
    pub async fn fetch_character(&self, name: &str) -> anyhow::Result<Option<LuroCharacter>> {
        Ok(self
            .db
            .get_user_character(self.user_id, name)
            .await?
            .map(|x| LuroCharacter::new(x, self.db.clone())))
    }
}
