use luro_model::user::character::CharacterImage;

impl crate::SQLxDriver {
    pub async fn update_character_image(&self, img: &CharacterImage) -> Result<CharacterImage, sqlx::Error> {
        sqlx::query_file_as!(
            CharacterImage,
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
