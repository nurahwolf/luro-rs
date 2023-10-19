use crate::{DbUserMarriage, LuroDatabase};

impl LuroDatabase {
    pub async fn get_marriages(&self, user_id: i64) -> Result<Vec<DbUserMarriage>, sqlx::Error> {
        sqlx::query_as!(
            DbUserMarriage,
            "SELECT * FROM user_marriages WHERE proposer_id = $1 or proposee_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await
    }
}
