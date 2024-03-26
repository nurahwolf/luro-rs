use crate::user::Marriage;

impl crate::database::sqlx::Database {
    pub async fn marriage_update(&self, marriage: Marriage) -> Result<u64, sqlx::Error> {
        sqlx::query_file!(
            "queries/marriage/marriage_update.sql",
            marriage.divorced,
            marriage.proposee_id.min(marriage.proposer_id).get() as i64,
            marriage.proposee_id.max(marriage.proposer_id).get() as i64,
            marriage.reason,
            marriage.rejected,
        )
        .execute(&self.pool)
        .await
        .map(|x| x.rows_affected())
    }
}
