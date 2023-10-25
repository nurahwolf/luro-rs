use sqlx::postgres::PgQueryResult;
use twilight_model::id::{marker::UserMarker, Id};

use crate::{LuroDatabase, LuroUserPermissions};

impl LuroDatabase {
    pub async fn update_user_permissions(&self, user_id: Id<UserMarker>, permissions: &LuroUserPermissions) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query_file!(
            "queries/users/update_user_permissions.sql",
            user_id.get() as i64,
            permissions as _,
        )
        .execute(&self.pool)
        .await
    }
}