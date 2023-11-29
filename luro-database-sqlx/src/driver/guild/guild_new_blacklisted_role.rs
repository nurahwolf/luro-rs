use twilight_model::id::{
    marker::{GuildMarker, RoleMarker},
    Id,
};

impl crate::SQLxDriver {
    pub async fn guild_new_blacklisted_role(&self, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) -> anyhow::Result<u64> {
        Ok(sqlx::query!(
            "
            INSERT INTO guild_role_blacklist(guild_id, role_id)
            VALUES ($1, $2)
            ",
            guild_id.get() as i64,
            role_id.get() as i64
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }
}
