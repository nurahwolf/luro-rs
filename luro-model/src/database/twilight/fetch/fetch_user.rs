use twilight_model::id::{marker::UserMarker, Id};

use crate::{
    database::twilight::{Database, Error},
    user::User,
};

impl Database {
    pub async fn fetch_user(&self, user_id: Id<UserMarker>) -> Result<User, Error> {
        let twilight_user = self.twilight_client.user(user_id).await?.model().await?;
        Ok(User::User(twilight_user.into()))
    }
}
