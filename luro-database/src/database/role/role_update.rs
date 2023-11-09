use luro_model::sync::RoleSync;

impl crate::Database {
    pub async fn role_update(&self, role: impl Into<RoleSync>) -> anyhow::Result<u64> {
        Ok(self.driver.update_role(role).await?)
    }
}