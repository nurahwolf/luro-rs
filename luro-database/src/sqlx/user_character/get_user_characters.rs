use crate::{sqlx::user_character::DbUserCharacter, LuroDatabase};

impl LuroDatabase {
    pub async fn get_user_characters(&self, user_id: i64) -> Result<Vec<DbUserCharacter>, sqlx::Error> {
        sqlx::query_as!(
            DbUserCharacter,
            "
            SELECT * FROM user_characters WHERE (user_id = $1)
            ",
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }
}
