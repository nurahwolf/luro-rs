use luro_model::user::character::CharacterImage;

use super::DbUserCharacter;

impl crate::SQLxDriver {
    pub async fn get_character_images(&self, character: &DbUserCharacter) -> Result<Vec<CharacterImage>, sqlx::Error> {
        sqlx::query_as!(
            CharacterImage,
            "
                SELECT
                    character_name,
                    favourite,
                    images.img_id,
                    name,
                    nsfw,
                    owner_id,
                    source,
                    url
                FROM images
                JOIN user_character_images ON images.img_id = user_character_images.img_id 
                WHERE
                    (user_id = $1 and character_name = $2)
            ",
            character.user_id,
            character.character_name,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_character_image(&self, character: &DbUserCharacter, img_id: i64) -> Result<Option<CharacterImage>, sqlx::Error> {
        sqlx::query_as!(
            CharacterImage,
            "
                SELECT
                    character_name,
                    favourite,
                    images.img_id,
                    name,
                    nsfw,
                    owner_id,
                    source,
                    url
                FROM images
                JOIN user_character_images second ON images.img_id = second.img_id 
                WHERE
                    (user_id = $1 and character_name = $2 and images.img_id = $3)
            ",
            character.user_id,
            character.character_name,
            img_id
        )
        .fetch_optional(&self.pool)
        .await
    }
}
