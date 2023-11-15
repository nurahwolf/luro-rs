use luro_model::user::character::CharacterImage;
use twilight_model::id::{marker::UserMarker, Id};

impl crate::SQLxDriver {
    pub async fn get_character_images(&self, user_id: Id<UserMarker>, character_name: &str) -> Result<Vec<CharacterImage>, sqlx::Error> {
        sqlx::query_file_as!(
            CharacterImage,
            "queries/character_fetch_images.sql",
            user_id.get() as i64,
            character_name,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_character_image(
        &self,
        user_id: Id<UserMarker>,
        character_name: &str,
        img_id: i64,
    ) -> Result<Option<CharacterImage>, sqlx::Error> {
        sqlx::query_file_as!(
            CharacterImage,
            "queries/character_fetch_image.sql",
            user_id.get() as i64,
            character_name,
            img_id
        )
        .fetch_optional(&self.pool)
        .await
    }
}
