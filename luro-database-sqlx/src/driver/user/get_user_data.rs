use luro_model::types::UserData;
use twilight_model::id::{marker::UserMarker, Id};

use crate::types::{DbGender, DbSexuality, DbUserPermissions};

impl crate::SQLxDriver {
    pub async fn get_user_data(&self, user_id: Id<UserMarker>) -> anyhow::Result<Option<UserData>> {
        Ok(sqlx::query_file!("queries/user/user_fetch_data.sql", user_id.get() as i64)
            .fetch_optional(&self.pool)
            .await
            .map(|x| {
                x.map(|data| UserData {
                    user_id: Id::new(data.user_id as u64),
                    permissions: data.user_permissions.into(),
                    gender: data.gender.map(|x| x.into()),
                    sexuality: data.sexuality.map(|x| x.into()),
                })
            })?)
    }
}
