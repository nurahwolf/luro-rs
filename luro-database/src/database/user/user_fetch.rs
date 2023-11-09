use luro_model::types::User;
use twilight_model::id::{marker::UserMarker, Id};

use crate::Database;

impl Database {
    pub async fn user_fetch(&self, user_id: Id<UserMarker>) -> anyhow::Result<User> {
        if let Ok(Some(user)) = self.driver.get_user(user_id).await {
            return Ok(user)
        }

        tracing::warn!("Failed to get user from database, falling back to twlight client");
        let twilight_user = self.api_client.user(user_id).await?.model().await?;
        if let Err(why) = self.driver.update_user(&twilight_user).await {
            tracing::error!(why = ?why, "Failed to sync user to the database");
        }

        Ok(twilight_user.into())
    }
}
