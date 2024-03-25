use crate::character::CharacterImage;

impl crate::database::sqlx::Database {
    pub async fn create_character_image(&self, img: &CharacterImage) -> Result<CharacterImage, sqlx::Error> {
        sqlx::query_file_as!(
            CharacterImage,
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
