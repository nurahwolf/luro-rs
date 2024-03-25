use twilight_model::id::{marker::UserMarker, Id};

use crate::{database::Error, user::UserContext};

impl crate::database::twilight::Database {
    pub async fn fetch_user(&self, user_id: Id<UserMarker>) -> Result<UserContext, Error> {
        let twilight_user = self.twilight_client.user(user_id).await?.model().await?;
        Ok(twilight_user.into())
    }
}
