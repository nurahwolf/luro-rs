use crate::LuroCharacterImage;
use crate::LuroDatabase;

use super::DbUserCharacter;

impl LuroDatabase {
    pub async fn get_character_images(&self, character: &DbUserCharacter) -> Result<Vec<LuroCharacterImage>, sqlx::Error> {
        sqlx::query_as!(
            LuroCharacterImage,
            "
                SELECT
                    character_name,
                    favourite,
                    first.img_id,
                    name,
                    nsfw,
                    owner_id,
                    source,
                    url
                FROM images first
                JOIN user_character_images second ON first.img_id = second.img_id 
                WHERE
                    (user_id = $1 and character_name = $2)
            ",
            character.user_id,
            character.character_name,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_character_image(&self, character: &DbUserCharacter, img_id: i64) -> Result<Option<LuroCharacterImage>, sqlx::Error> {
        sqlx::query_as!(
            LuroCharacterImage,
            "
                SELECT
                    character_name,
                    favourite,
                    first.img_id,
                    name,
                    nsfw,
                    owner_id,
                    source,
                    url
                FROM images first
                JOIN user_character_images second ON first.img_id = second.img_id 
                WHERE
                    (user_id = $1 and character_name = $2 and first.img_id = $3)
            ",
            character.user_id,
            character.character_name,
            img_id
        )
        .fetch_optional(&self.pool)
        .await
    }
}
