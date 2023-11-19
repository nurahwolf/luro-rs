use super::DbUserMarriage;

impl crate::SQLxDriver {
    pub async fn delete_marriage(&self, user_id: (i64, i64)) -> Result<Option<DbUserMarriage>, sqlx::Error> {
        let proposee_id = user_id.0.min(user_id.1);
        let proposer_id = user_id.0.max(user_id.1);
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
            proposee_id,
            proposer_id
        )
        .fetch_optional(&self.pool)
        .await
    }
}
