use sqlx::postgres::PgQueryResult;
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker},
    Id,
};

use crate::{luro::luro_role_data::LuroRoleData, LuroDatabase};

impl LuroDatabase {
    pub async fn update_role_data(
        &self,
        data: LuroRoleData,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query_file!(
            "queries/guild_roles/update_role_data.sql",
            data.deleted,
            guild_id.get() as i64,
            role_id.get() as i64,
        )
        .execute(&self.pool)
        .await
    }
}
