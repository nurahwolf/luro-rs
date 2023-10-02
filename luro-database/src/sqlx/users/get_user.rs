use luro_model::user::LuroUser;
use sqlx::Error;

use crate::{DatabaseUser, LuroDatabase, LuroUserPermissions};

impl LuroDatabase {
    pub async fn get_user(&self, id: i64) -> Result<Option<LuroUser>, Error> {
        let query = sqlx::query_as!(
            DatabaseUser,
            "SELECT
                name,
                user_id,
                user_permissions as \"user_permissions: LuroUserPermissions\"
            FROM
                users
            WHERE
                user_id = $1",
            id
        );

        query.fetch_optional(&self.0).await.map(|x| x.map(|x| x.luro_user()))
    }
}
