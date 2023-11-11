use luro_model::user::character::CharacterProfile;
use twilight_model::id::{marker::UserMarker, Id};

impl crate::Database {
    pub async fn user_fetch_characters(&self, user_id: Id<UserMarker>) -> anyhow::Result<Vec<CharacterProfile>> {
        Ok(self.driver.get_user_characters(user_id).await?)
    }
}
