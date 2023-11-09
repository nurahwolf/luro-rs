impl crate::SQLxDriver {
    pub async fn count_guild_roles(&self) -> Result<i64, sqlx::Error> {
        sqlx::query!(
            "
        SELECT 
            COUNT(*) as count
        FROM 
            guild_roles
        "
        )
        .fetch_one(&self.pool)
        .await
        .map(|x| x.count.unwrap_or_default())
    }
}
