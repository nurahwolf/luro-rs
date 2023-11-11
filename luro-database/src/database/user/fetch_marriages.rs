use luro_model::user::marriage::Marriage;
use twilight_model::id::{Id, marker::UserMarker};

impl crate::Database {
    pub async fn user_fetch_marriages(&self, user_id: Id<UserMarker>) -> anyhow::Result<Vec<Marriage>> {
        Ok(self.driver.user_fetch_marriages(user_id).await?)
    }
}