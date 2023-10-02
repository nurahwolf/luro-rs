use luro_model::user::LuroUser;
use sqlx::Error;

use crate::{LuroDatabase, DatabaseUser, LuroUserPermissions};

impl LuroDatabase {
    pub async fn update_user(&self, user: impl Into<DatabaseUser>) -> Result<Option<LuroUser>, Error> {
        let user = user.into();

        let query = sqlx::query_as!(
            DatabaseUser,
            "INSERT INTO users (
                accent_colour,
                user_id,
                user_permissions,
                name
            ) VALUES
                ($1, $2, $3, $4)
            ON CONFLICT
                (user_id)
            DO UPDATE SET
                name = $3
            RETURNING
                accent_colour,
                user_id,
                user_permissions as \"user_permissions: LuroUserPermissions\",
                name",
            user.accent_colour as _,
            user.user_id as _,
            user.user_permissions as _,
            user.name as _,
        );

        query.fetch_optional(&self.0).await.map(|x| x.map(|x| x.luro_user()))
    }
}