use twilight_model::id::{marker::UserMarker, Id};

use crate::{database::Error, user::UserContext};

impl crate::database::Database {
    pub async fn fetch_user(&self, user_id: Id<UserMarker>) -> Result<UserContext, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.fetch_user(user_id).await {
            Ok(Some(user)) => return Ok(user),
            Ok(None) => tracing::warn!("The user `{user_id}` was requested from the database, but the user was not present."),
            Err(why) => tracing::error!(?why, "Database failed to fetch user `{user_id}`, falling back to Twilight."),
        };

        Ok(self.twilight_client.user(user_id).await?.model().await.map(|x| x.into())?)
    }
}
