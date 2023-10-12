use sqlx::Error;

use crate::{DatabaseUserType, LuroDatabase, DatabaseUser};

impl LuroDatabase {
    pub async fn update_user(&self, user: impl Into<DatabaseUserType>) -> Result<Option<DatabaseUser>, Error> {
        let user = user.into();

        match user {
            DatabaseUserType::User(user) => self.handle_user(user).await,
            DatabaseUserType::LuroUser(user) => self.handle_luro_user(user).await,
            DatabaseUserType::UserUpdate(user) => self.handle_user_update(user).await,
        }
    }
}
