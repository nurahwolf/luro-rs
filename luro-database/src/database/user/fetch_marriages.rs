use luro_model::user::marriages::UserMarriage;
use twilight_model::id::{Id, marker::UserMarker};

impl crate::Database {
    pub async fn user_fetch_marriages(&self, user_id: Id<UserMarker>) -> anyhow::Result<Vec<UserMarriage>> {
        Ok(self.driver.user_fetch_marriages(user_id).await?)
    }
}