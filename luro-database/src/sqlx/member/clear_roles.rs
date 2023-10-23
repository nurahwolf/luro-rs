use sqlx::postgres::PgQueryResult;
use twilight_model::id::{marker::{UserMarker, GuildMarker}, Id};

use crate::LuroDatabase;

impl LuroDatabase {
    pub async fn clear_member_roles(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query_file!(
            "queries/guild_member_roles/clear_roles.sql",
            guild_id.get() as i64,
            user_id.get() as i64,
        )
        .execute(&self.pool)
        .await
    }
}