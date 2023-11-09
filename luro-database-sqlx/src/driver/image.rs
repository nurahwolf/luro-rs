use luro_model::types::Image;

impl crate::SQLxDriver {
    /// Fetches ALL images, including NSFW
    pub async fn fetch_images(&self) -> Result<Vec<Image>, sqlx::Error> {
        sqlx::query_as!(
            Image,
            "
            SELECT *
            FROM images
            "
        )
        .fetch_all(&self.pool)
        .await
    }

    /// Fetches ALL speciefied image type
    pub async fn fetch_images_nsfw(&self, nsfw: bool) -> Result<Vec<Image>, sqlx::Error> {
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

    /// Uploads a new image
    pub async fn update_image(&self, img: Image) -> Result<Image, sqlx::Error> {
        sqlx::query_as!(
            Image,
            "
            UPDATE images
            SET
                name = $2,
                nsfw = $3,
                owner_id = $4,
                source = $5,
                url = $6
            WHERE img_id = $1
            RETURNING *
            ",
            img.img_id,
            img.name,
            img.nsfw,
            img.owner_id,
            img.source,
            img.url,
        )
        .fetch_one(&self.pool)
        .await
    }

    /// Uploads a new image
    pub async fn new_image(&self, img: Image) -> Result<Image, sqlx::Error> {
        sqlx::query_as!(
            Image,
            "
            INSERT INTO images (
                name,
                nsfw,
                owner_id,
                source,
                url
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (img_id)
            DO NOTHING
            RETURNING *
            ",
            img.name,
            img.nsfw,
            img.owner_id,
            img.source,
            img.url,
        )
        .fetch_one(&self.pool)
        .await
    }
}
