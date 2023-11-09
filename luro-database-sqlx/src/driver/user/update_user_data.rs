use crate::types::{DbGender, DbSexuality, DbUserPermissions};

impl crate::SQLxDriver {
    pub async fn update_user_data(&self, user_id: twilight_model::id::Id<twilight_model::id::marker::UserMarker>, data: luro_model::types::UserData) -> anyhow::Result<u64> {
        let gender: Option<DbGender> = data.gender.map(|x|x.into());
        let sexuality: Option<DbSexuality> = data.sexuality.map(|x|x.into());
        let permissions: DbUserPermissions = data.permissions.into();

        Ok(sqlx::query_file!(
            "queries/luro_user/update_user_data.sql",
            user_id.get() as i64,
            gender as _,
            sexuality as _,
            permissions as _,
        )
        .execute(&self.pool)
        .await?.rows_affected())
    }
}
