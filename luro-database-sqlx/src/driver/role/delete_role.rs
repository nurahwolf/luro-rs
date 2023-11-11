impl crate::SQLxDriver {
    pub async fn role_delete(&self, role_id: i64) -> Result<u64, sqlx::Error> {
        sqlx::query_as!(
            DbRole,
            "INSERT INTO guild_roles (
                deleted,
                role_id
            ) VALUES
                ($1, $2)
            ON CONFLICT
                (role_id, guild_id)
            DO UPDATE SET
                deleted = $1",
            true,
            role_id,
        )
        .execute(&self.pool)
        .await
        .map(|x| x.rows_affected())
    }
}
