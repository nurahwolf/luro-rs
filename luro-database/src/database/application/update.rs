use luro_model::sync::ApplicationSync;

impl crate::Database {
    pub async fn application_update(&self, data: impl Into<ApplicationSync>) -> anyhow::Result<u64> {
        Ok(self.driver.update_application(data).await?)
    }
}