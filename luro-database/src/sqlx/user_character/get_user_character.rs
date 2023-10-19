use crate::{sqlx::user_character::DbUserCharacter, LuroDatabase};

impl LuroDatabase {
    pub async fn get_user_character(&self, user_id: i64, name: &str) -> Result<Option<DbUserCharacter>, sqlx::Error> {
        sqlx::query_as!(
            DbUserCharacter,
            "
            SELECT * FROM user_characters WHERE (user_id = $1 and character_name = $2)
            ",
            user_id,
            name
        )
        .fetch_optional(&self.pool)
        .await
    }
}
