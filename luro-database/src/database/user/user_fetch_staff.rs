use luro_model::types::User;

impl crate::Database {
    pub async fn user_fetch_staff(&self) -> anyhow::Result<Vec<User>> {
        self.driver.get_staff().await
    }
}