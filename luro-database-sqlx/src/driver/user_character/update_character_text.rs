use super::{DbUserCharacter};

impl crate::SQLxDriver {
    pub async fn update_user_character_text(&self, character: DbUserCharacter) -> Result<DbUserCharacter, sqlx::Error> {
        sqlx::query_as!(
            DbUserCharacter,
            "
            INSERT INTO user_characters (
                character_name,
                nsfw_description,
                nsfw_summary,
                sfw_description,
                sfw_summary,
                user_id
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (user_id, character_name)
            DO UPDATE SET
                nsfw_description = $2,
                nsfw_summary = $4,
                sfw_description = $5,
                sfw_summary = $6
            RETURNING *
            ",
            character.character_name,
            character.nsfw_description,
            character.nsfw_summary,
            character.sfw_description,
            character.sfw_summary,
            character.user_id,
        )
        .fetch_one(&self.pool)
        .await
    }
}
