impl crate::SQLxDriver {
    pub async fn count_guild_members(&self) -> Result<i64, sqlx::Error> {
        sqlx::query!(
            "
        SELECT 
            COUNT(*) as count
        FROM 
            guild_members
        "
        )
        .fetch_one(&self.pool)
        .await
        .map(|x| x.count.unwrap_or_default())
    }
}
