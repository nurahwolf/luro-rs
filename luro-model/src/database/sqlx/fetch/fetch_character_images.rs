use twilight_model::id::{marker::UserMarker, Id};

use crate::character::CharacterImage;

impl crate::database::sqlx::Database {
    pub async fn fetch_character_images(
        &self,
        user_id: Id<UserMarker>,
        character_name: &str,
    ) -> Result<Vec<CharacterImage>, sqlx::Error> {
        sqlx::query_file_as!(
            CharacterImage,
            "queries/fetch/character_images.sql",
            user_id.get() as i64,
            character_name,
        )
        .fetch_all(&self.pool)
        .await
    }
}
