use luro_model::types::Image;

impl crate::Database {
    pub async fn image_new(&self, img: Image) -> anyhow::Result<Image> {
        Ok(self.driver.new_image(img).await?)
    }
}
