use sqlx::postgres::PgQueryResult;
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;

use crate::LuroDatabase;
use crate::LuroUserData;

impl LuroDatabase {
    pub async fn update_user_data(&self, user_id: Id<UserMarker>, data: &LuroUserData) -> anyhow::Result<PgQueryResult> {
        Ok(sqlx::query_file!(
            "queries/luro_user/update_user_data.sql",
            user_id.get() as i64,
            data.gender as _,
            data.sexuality as _,
            data.permissions as _,
        )
        .execute(&self.pool)
        .await?)
    }
}
