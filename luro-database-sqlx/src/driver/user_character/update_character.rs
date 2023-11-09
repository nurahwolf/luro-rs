use super::{DbUserCharacter};

impl crate::SQLxDriver {
    pub async fn update_user_character(&self, character: DbUserCharacter) -> Result<DbUserCharacter, sqlx::Error> {
        sqlx::query_as!(
            DbUserCharacter,
            "
            INSERT INTO user_characters (
                character_name,
                nsfw_description,
                nsfw_icons,
                nsfw_summary,
                prefix,
                sfw_description,
                sfw_icons,
                sfw_summary,
                user_id
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7 ,$8, $9)
            ON CONFLICT (user_id, character_name)
            DO UPDATE SET
                nsfw_description = $2,
                nsfw_icons = $3,
                nsfw_summary = $4,
                prefix = $5,
                sfw_description = $6,
                sfw_icons = $7,
                sfw_summary = $8
            RETURNING *
            ",
            character.character_name,
            character.nsfw_description,
            character.nsfw_icons.as_deref(),
            character.nsfw_summary,
            character.prefix,
            character.sfw_description,
            character.sfw_icons.as_deref(),
            character.sfw_summary,
            character.user_id,
        )
        .fetch_one(&self.pool)
        .await
    }
}
