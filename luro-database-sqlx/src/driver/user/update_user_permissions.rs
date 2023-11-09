use sqlx::postgres::PgQueryResult;

use crate::types::DbUserPermissions;

impl crate::SQLxDriver {
    pub async fn update_user_permissions(
        &self,
        user_id: twilight_model::id::Id<twilight_model::id::marker::UserMarker>,
        permissions: impl Into<DbUserPermissions>,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query_file!("queries/users/update_user_permissions.sql", user_id.get() as i64, permissions.into() as _,)
            .execute(&self.pool)
            .await
    }
}
