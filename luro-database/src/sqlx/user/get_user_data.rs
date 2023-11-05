use twilight_model::id::{marker::UserMarker, Id};

use crate::{Gender, LuroDatabase, LuroUserData, LuroUserPermissions, Sexuality};

impl LuroDatabase {
    pub async fn get_user_data(&self, user_id: Id<UserMarker>) -> anyhow::Result<Option<LuroUserData>> {
        Ok(sqlx::query_file!("queries/luro_user/get_user_data.sql", user_id.get() as i64)
            .fetch_optional(&self.pool)
            .await
            .map(|x| {
                x.map(|data| LuroUserData {
                    permissions: data.user_permissions,
                    gender: data.gender,
                    sexuality: data.sexuality,
                })
            })?)
    }
}
