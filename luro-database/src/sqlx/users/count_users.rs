impl crate::LuroDatabase {
    pub async fn count_users(&self) -> Result<i64, sqlx::Error> {
        let query = sqlx::query!("
        SELECT 
            COUNT(*) as count
        FROM 
            users
        ").fetch_all(&self.0).await?;

        let result = query.into_iter().map(|x|x.count.unwrap_or_default()).collect::<Vec<_>>();
        Ok(result.first().copied().unwrap_or_default())
    }
}