use crate::image::Image;

impl crate::database::sqlx::Database {
    pub async fn fetch_images(&self, nsfw: bool) -> Result<Vec<Image>, sqlx::Error> {
        sqlx::query_as!(
            Image,
            "
            SELECT *
            FROM images
            WHERE nsfw = $1
            ",
            nsfw
        )
        .fetch_all(&self.pool)
        .await
    }
}
