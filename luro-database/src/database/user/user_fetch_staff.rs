use luro_model::types::User;

impl crate::Database {
    pub async fn user_fetch_staff(&self) -> anyhow::Result<Vec<User>> {
        match self.driver.user_fetch_staff().await {
            Ok(users) => Ok(users),
            Err(why) => {
                tracing::error!(why = ?why, "Database failed to return any staff members");
                Ok(vec![])
            }
        }
    }
}
