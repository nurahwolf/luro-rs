use twilight_model::id::{marker::UserMarker, Id};

use crate::{sqlx::user_character::DbUserCharacter, LuroDatabase};

impl LuroDatabase {
    pub async fn get_user_characters(&self, user_id: Id<UserMarker>) -> Result<Vec<DbUserCharacter>, sqlx::Error> {
        sqlx::query_as!(
            DbUserCharacter,
            "
            SELECT * FROM user_characters WHERE (user_id = $1)
            ",
            user_id.get() as i64
        )
        .fetch_all(&self.pool)
        .await
    }
}
