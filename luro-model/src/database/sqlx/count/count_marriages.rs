impl crate::database::sqlx::Database {
    pub async fn count_marriages(&self) -> Result<i64, sqlx::Error> {
        sqlx::query!(
            "
            SELECT
                COUNT(*) as count
            FROM
                user_marriages
        "
        )
        .fetch_one(&self.pool)
        .await
        .map(|x| x.count.unwrap_or_default())
    }
}
