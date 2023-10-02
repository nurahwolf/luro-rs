use luro_model::user::LuroUser;
use sqlx::Error;
use twilight_model::user::User;

use crate::{DatabaseUser, LuroDatabase, LuroUserPermissions};

impl LuroDatabase {
    pub async fn handle_user(&self, user: User) -> Result<Option<LuroUser>, Error> {
        let query = sqlx::query_as!(
            DatabaseUser,
            "INSERT INTO users (
                user_id,
                user_permissions,
                name
            ) VALUES
                ($1, $2, $3)
            ON CONFLICT
                (user_id)
            DO UPDATE SET
                name = $3
            RETURNING
                user_id,
                user_permissions as \"user_permissions: LuroUserPermissions\",
                name",
            user.id.get() as i64,
            LuroUserPermissions::default() as _,
            user.name
        );

        query.fetch_optional(&self.0).await.map(|x| x.map(|x| x.luro_user()))
    }
}
