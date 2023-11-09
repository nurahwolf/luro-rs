impl crate::SQLxDriver {
    pub async fn count_applications(&self) -> Result<i64, sqlx::Error> {
        sqlx::query!(
            "
        SELECT 
            COUNT(*) as count
        FROM 
            applications
        "
        )
        .fetch_one(&self.pool)
        .await
        .map(|x| x.count.unwrap_or_default())
    }
}
