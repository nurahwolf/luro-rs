use super::DbUserMarriage;

impl crate::SQLxDriver {
    pub async fn delete_marriage(&self, user_id: (i64, i64)) -> Result<Option<DbUserMarriage>, sqlx::Error> {
        sqlx::query_as!(
            DbUserMarriage,
            "
            DELETE FROM user_marriages
            WHERE (proposer_id, proposee_id) IN (select proposer_id, proposee_id from user_marriages where
                (proposer_id = $1 AND proposee_id = $2)
                    or
                (proposer_id = $2 AND proposee_id = $1)
            )
            RETURNING *
            ",
            user_id.0,
            user_id.1
        )
        .fetch_optional(&self.pool)
        .await
    }
}
