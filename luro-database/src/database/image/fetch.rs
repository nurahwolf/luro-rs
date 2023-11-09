use luro_model::types::Image;

impl crate::Database {
    pub async fn images_fetch(&self, nsfw: bool) -> anyhow::Result<Vec<Image>> {
        Ok(self.driver.fetch_images_nsfw(nsfw).await?)
    }
}
