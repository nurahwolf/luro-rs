use crate::{LuroDatabase, LuroMemberData};

impl LuroDatabase {
    /// Updates a supported member type. Returns the total number of rows modified in the database.
    pub async fn update_member_data(&self, data: &LuroMemberData) -> anyhow::Result<u64> {
        Ok(sqlx::query_file!(
            "queries/luro_user/update_member_data.sql",
            data.guild_id.get() as i64,
            data.left_at,
            data.user_id.get() as i64
        )
        .execute(&self.pool)
        .await
        .map(|x| x.rows_affected())?)
    }
}
