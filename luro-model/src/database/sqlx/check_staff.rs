use twilight_model::id::{marker::UserMarker, Id};

use crate::{
    database::sqlx::{Database, Error},
    user::User,
};

impl Database {
    pub async fn check_staff(&self, user_id: Id<UserMarker>) -> Result<Option<User>, Error> {
        let staff = self.fetch_staff().await?;

        Ok(staff.into_iter().find(|staff| staff.user_id() == user_id))
    }
}
