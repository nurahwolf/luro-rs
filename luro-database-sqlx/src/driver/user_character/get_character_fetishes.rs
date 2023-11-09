use super::{DbUserCharacter, DbCharacterFetish, DbUserFetishCategory};

impl crate::SQLxDriver {
    pub async fn get_character_fetishes(&self, character: &DbUserCharacter) -> Result<Vec<DbCharacterFetish>, sqlx::Error> {
        sqlx::query_as!(
            DbCharacterFetish,
            "
                SELECT
                    category as \"category: DbUserFetishCategory\",
                    character_name,
                    character_fetish.fetish_id,
                    user_id,
                    name,
                    description
                FROM user_characters_fetishes character_fetish
                JOIN fetishes fetish_details ON character_fetish.fetish_id = fetish_details.fetish_id 
                WHERE
                    (user_id = $1 and character_name = $2)
            ",
            character.user_id,
            character.character_name,
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_character_fetish(
        &self,
        character: &DbUserCharacter,
        fetish_id: i64,
    ) -> Result<Option<DbCharacterFetish>, sqlx::Error> {
        sqlx::query_as!(
            DbCharacterFetish,
            "
                SELECT
                    category as \"category: DbUserFetishCategory\",
                    character_name,
                    character_fetish.fetish_id,
                    user_id,
                    name,
                    description
                FROM user_characters_fetishes character_fetish
                JOIN fetishes fetish_details ON character_fetish.fetish_id = fetish_details.fetish_id 
                WHERE
                    (user_id = $1 and character_name = $2 and character_fetish.fetish_id = $3)
            ",
            character.user_id,
            character.character_name,
            fetish_id,
        )
        .fetch_optional(&self.pool)
        .await
    }
}
