use crate::{DbUserMarriage, LuroDatabase};

impl LuroDatabase {
    pub async fn get_marriage(&self, user_id: (i64, i64)) -> Result<Option<DbUserMarriage>, sqlx::Error> {
        sqlx::query_as!(
            DbUserMarriage,
            "
            SELECT * FROM user_marriages WHERE
                (proposer_id = $1 AND proposee_id = $2)
                    OR
                (proposer_id = $2 AND proposee_id = $1)
            ",
            user_id.0,
            user_id.1
        )
        .fetch_optional(&self.pool)
        .await
    }
}
