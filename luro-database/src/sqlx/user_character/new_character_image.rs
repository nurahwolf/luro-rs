use crate::LuroCharacterImage;
use crate::LuroDatabase;

impl LuroDatabase {
    pub async fn new_character_image(&self, img: &LuroCharacterImage) -> Result<LuroCharacterImage, sqlx::Error> {
        sqlx::query_file_as!(
            LuroCharacterImage,
            "queries/user_characters/new_character_image.sql",
            img.character_name,
            img.favourite,
            img.name,
            img.nsfw,
            img.owner_id,
            img.source,
            img.url
        )
        .fetch_one(&self.pool)
        .await
    }
}
