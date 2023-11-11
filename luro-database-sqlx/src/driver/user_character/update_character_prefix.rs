use super::DbUserCharacter;

impl crate::SQLxDriver {
    pub async fn update_user_character_prefix(&self, character: DbUserCharacter) -> Result<DbUserCharacter, sqlx::Error> {
        sqlx::query_as!(
            DbUserCharacter,
            "
            INSERT INTO user_characters (
                character_name,
                prefix,
                user_id
            )
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, character_name)
            DO UPDATE SET
                prefix = $2
            RETURNING *
            ",
            character.character_name,
            character.prefix,
            character.user_id,
        )
        .fetch_one(&self.pool)
        .await
    }
}
