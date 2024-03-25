use twilight_model::id::{marker::UserMarker, Id};

use crate::character::CharacterImage;

impl crate::database::sqlx::Database {
    pub async fn fetch_character_image(
        &self,
        user_id: Id<UserMarker>,
        character_name: &str,
        img_id: i64,
    ) -> Result<Option<CharacterImage>, sqlx::Error> {
        sqlx::query_file_as!(
            CharacterImage,
            "queries/fetch/character_image.sql",
            user_id.get() as i64,
            character_name,
            img_id
        )
        .fetch_optional(&self.pool)
        .await
    }
}
