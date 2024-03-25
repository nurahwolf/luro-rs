use crate::character::Character;

impl crate::database::sqlx::Database {
    pub async fn update_character(&self, character: &Character<'_>) -> Result<(), sqlx::Error> {
        sqlx::query_file!(
            "queries/update/character.sql",
            character.name,
            character.nsfw_description,
            character.nsfw_summary,
            character.prefix,
            character.sfw_description,
            character.sfw_summary,
            character.user_id.get() as i64,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
