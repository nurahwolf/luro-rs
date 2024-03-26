impl crate::database::sqlx::Database {
    pub async fn count_channels(&self) -> Result<i64, sqlx::Error> {
        sqlx::query!(
            "
        SELECT
            COUNT(*) as count
        FROM
            channels
        "
        )
        .fetch_one(&self.pool)
        .await
        .map(|x| x.count.unwrap_or_default())
    }
}
