use luro_model::user::marriage::Marriage;

impl crate::SQLxDriver {
    pub async fn marriage_update(&self, marriage: Marriage) -> anyhow::Result<u64> {
        Ok(sqlx::query!(
            "
            INSERT INTO user_marriages (divorced, proposee_id, proposer_id, reason, rejected)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (proposer_id, proposee_id)
            DO UPDATE SET divorced = $1, proposer_id = $2, proposee_id = $3, reason = $4, rejected = $5
            ",
            marriage.divorced,
            marriage.proposee_id.get() as i64,
            marriage.proposer_id.get() as i64,
            marriage.reason,
            marriage.rejected,
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }
}
