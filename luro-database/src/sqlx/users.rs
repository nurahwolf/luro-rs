use std::collections::HashMap;

use futures_util::TryStreamExt;
use luro_model::user::LuroUser;
use sqlx::Error;
use twilight_model::{
    id::{marker::UserMarker, Id},
    user::User,
};

use crate::{DatabaseUser, LuroDatabase, LuroUserPermissions};

impl DatabaseUser {
    pub fn luro_user(&self) -> LuroUser {
        let mut luro_user = LuroUser::new(Id::new(self.user_id as u64));
        luro_user.name = self.name.clone();
        luro_user.user_permissions = match self.user_permissions {
            LuroUserPermissions::User => luro_model::user::LuroUserPermissions::User,
            LuroUserPermissions::Owner => luro_model::user::LuroUserPermissions::Owner,
            LuroUserPermissions::Administrator => luro_model::user::LuroUserPermissions::Administrator,
        };
        luro_user
    }
}

impl LuroDatabase {
    pub async fn get_user(&self, id: i64) -> Result<Option<LuroUser>, Error> {
        let query = sqlx::query_as!(
            DatabaseUser,
            r#"SELECT user_id, user_permissions as "user_permissions: LuroUserPermissions", name FROM users WHERE user_id = $1"#,
            id
        );

        query.fetch_optional(&self.0).await.map(|x| x.map(|x| x.luro_user()))
    }

    pub async fn get_users(&self) -> HashMap<Id<UserMarker>, LuroUser> {
        let mut users = HashMap::new();
        let query = sqlx::query_as!(
            DatabaseUser,
            r#"SELECT user_id, user_permissions as "user_permissions: LuroUserPermissions", name FROM users"#,
        );

        for user in (query.fetch(&self.0).try_next().await).into_iter().flatten() {
            users.insert(Id::new(user.user_id as u64), user.luro_user());
        }

        users
    }

    pub async fn get_staff(&self) -> HashMap<Id<UserMarker>, LuroUser> {
        let mut users = HashMap::new();
        let mut query = sqlx::query_as!(
            DatabaseUser,
            r#"SELECT user_id, user_permissions as "user_permissions: LuroUserPermissions", name FROM users WHERE user_permissions = 'OWNER' or  user_permissions = 'ADMINISTRATOR'"#,
        ).fetch(&self.0);

        while let Ok(Some(user)) = query.try_next().await {
            users.insert(Id::new(user.user_id as u64), user.luro_user());
        }

        users
    }

    pub async fn update_user(&self, user: impl Into<LuroUser>) -> Result<LuroUser, Error> {
        let user = user.into();
        let query = sqlx::query_as!(
            DatabaseUser,
            r#"INSERT INTO users (user_id, user_permissions, name) VALUES ($1, $2, $3) ON CONFLICT (user_id) DO UPDATE SET user_permissions = $2, name = $3 RETURNING user_id, user_permissions as "user_permissions: LuroUserPermissions", name"#,
            user.id.get() as i64,
            LuroUserPermissions::default() as _,
            user.name
        );

        query.fetch_one(&self.0).await.map(|x| x.luro_user())
    }

    pub async fn register_staff(&self, user: User) -> Result<LuroUser, Error> {
        let query = sqlx::query_as!(
            DatabaseUser,
            r#"INSERT INTO users (user_id, user_permissions, name) VALUES ($1, $2, $3) ON CONFLICT (user_id) DO UPDATE SET user_permissions = $2, name = $3 RETURNING user_id, user_permissions as "user_permissions: LuroUserPermissions", name"#,
            user.id.get() as i64,
            LuroUserPermissions::Administrator as _,
            user.name
        );

        query.fetch_one(&self.0).await.map(|x| x.luro_user())
    }

    pub async fn register_owner(&self, user: User) -> Result<LuroUser, Error> {
        let query = sqlx::query_as!(
            DatabaseUser,
            r#"INSERT INTO users (user_id, user_permissions, name) VALUES ($1, $2, $3) ON CONFLICT (user_id) DO UPDATE SET user_permissions = $2, name = $3 RETURNING user_id, user_permissions as "user_permissions: LuroUserPermissions", name"#,
            user.id.get() as i64,
            LuroUserPermissions::Owner as _,
            user.name
        );

        query.fetch_one(&self.0).await.map(|x| x.luro_user())
    }
}
