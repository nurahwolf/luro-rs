impl crate::SQLxDriver {
    pub async fn marriage_update(&self, marriage: luro_model::types::Marriage) -> anyhow::Result<u64> {
        let proposee_id = marriage.proposee_id.min(marriage.proposer_id).get() as i64;
        let proposer_id = marriage.proposee_id.max(marriage.proposer_id).get() as i64;

        Ok(sqlx::query_file!(
            "queries/marriage/marriage_update.sql",
            marriage.divorced,
            proposee_id,
            proposer_id,
            marriage.reason,
            marriage.rejected,
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }
}
