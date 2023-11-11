use luro_model::user::character::CharacterProfile;
use twilight_model::id::{marker::UserMarker, Id};

impl crate::Database {
    pub async fn user_fetch_character(&self, user_id: Id<UserMarker>, name: &str) -> anyhow::Result<Option<CharacterProfile>> {
        Ok(self.driver.get_user_character(user_id, name).await?)
    }
}
