use twilight_model::id::{
    marker::{GuildMarker, RoleMarker, UserMarker},
    Id,
};

impl crate::database::sqlx::Database {
    pub async fn update_member_role(
        &self,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
        user_id: Id<UserMarker>,
    ) -> Result<u64, sqlx::Error> {
        sqlx::query_file!(
            "queries/member/member_update_role.sql",
            guild_id.get() as i64,
            role_id.get() as i64,
            user_id.get() as i64,
        )
        .execute(&self.pool)
        .await
        .map(|x| x.rows_affected())
    }
}
