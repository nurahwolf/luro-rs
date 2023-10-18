impl crate::LuroDatabase {
    pub async fn count_messages(&self) -> Result<i64, sqlx::Error> {
        sqlx::query!(
            "
        SELECT 
            COUNT(*) as count
        FROM 
            messages
        "
        )
        .fetch_one(&self.pool)
        .await
        .map(|x| x.count.unwrap_or_default())
    }
}
