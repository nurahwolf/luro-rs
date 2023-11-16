use twilight_model::id::{marker::UserMarker, Id};

impl crate::Database {
    pub async fn user_fetch_marriages(&self, user_id: Id<UserMarker>) -> anyhow::Result<Vec<luro_model::types::Marriage>> {
        Ok(self.driver.user_fetch_marriages(user_id).await?)
    }
}
