impl crate::SQLxDriver {
    pub async fn count_users(&self) -> Result<i64, sqlx::Error> {
        sqlx::query!(
            "
        SELECT 
            COUNT(*) as count
        FROM 
            users
        "
        )
        .fetch_one(&self.pool)
        .await.map(|x| x.count.unwrap_or_default())
    }
}
