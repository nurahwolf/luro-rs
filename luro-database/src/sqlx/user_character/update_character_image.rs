use crate::LuroCharacterImage;
use crate::LuroDatabase;

impl LuroDatabase {
    pub async fn update_character_image(&self, img: &LuroCharacterImage) -> Result<LuroCharacterImage, sqlx::Error> {
        sqlx::query_file_as!(
            LuroCharacterImage,
            "queries/user_characters/update_character_image.sql",
            img.character_name,
            img.favourite,
            img.img_id,
            img.name,
            img.nsfw,
            img.owner_id,
            img.source as _,
            img.url
        )
        .fetch_one(&self.pool)
        .await
    }
}
