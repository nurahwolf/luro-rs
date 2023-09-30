use sqlx::Error;

use crate::user::LuroUser;

use super::PostgresDriver;
#[derive(Default, sqlx::Type)]
#[sqlx(type_name = "user_permissions", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuroUserPermissions {
    #[default]
    User,
    Owner,
    Administrator,
}
pub struct DatabaseUser {
    pub name: String,
    pub user_id: i64,
    pub user_permissions: LuroUserPermissions,
}

impl PostgresDriver {
    pub async fn get_user(&self, id: i64) -> Result<Option<DatabaseUser>, Error> {
        let query = sqlx::query_as!(
            DatabaseUser,
            r#"SELECT user_id, user_permissions as "user_permissions: LuroUserPermissions", name FROM users WHERE user_id = $1"#,
            id
        );

        query.fetch_optional(&self.0).await
    }

    pub async fn get_users(&self) -> Result<Vec<DatabaseUser>, Error> {
        let query = sqlx::query_as!(
            DatabaseUser,
            r#"SELECT user_id, user_permissions as "user_permissions: LuroUserPermissions", name FROM users"#,
        );

        query.fetch_all(&self.0).await
    }

    pub async fn get_staff(&self) -> Result<Vec<DatabaseUser>, Error> {
        let query = sqlx::query_as!(
            DatabaseUser,
            r#"SELECT user_id, user_permissions as "user_permissions: LuroUserPermissions", name FROM users WHERE user_permissions = 'OWNER'"#,
        );

        query.fetch_all(&self.0).await
    }

    pub async fn update_user(&self, user: impl Into<LuroUser>) -> Result<DatabaseUser, Error> {
        let user = user.into();
        let query = sqlx::query_as!(
            DatabaseUser,
            r#"INSERT INTO users (user_id, user_permissions, name) VALUES ($1, $2, $3) ON CONFLICT (user_id) DO UPDATE SET user_id = $1, name = $3 RETURNING user_id, user_permissions as "user_permissions: LuroUserPermissions", name"#,
            user.id.get() as i64,
            LuroUserPermissions::default() as _,
            user.name
        );

        query.fetch_one(&self.0).await
    }
}
