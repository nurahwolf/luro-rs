use twilight_model::id::{marker::GuildMarker, Id};

use crate::{database::Error, guild::Guild};

impl crate::database::Database {
    pub async fn fetch_guild(&self, guild_id: Id<GuildMarker>) -> Result<Guild, Error> {
        #[cfg(feature = "database-sqlx")]
        // match fetch_user(self, user_id).await {
        //     Ok(Some(user)) => return Ok(user),
        //     Ok(None) => tracing::warn!("The user `{user_id}` was requested from the database, but the user was not present."),
        //     Err(why) => tracing::error!(?why, "Database failed to fetch user `{user_id}`, falling back to Twilight."),
        // };
        Ok(self
            .twilight_client
            .guild(guild_id)
            .await?
            .model()
            .await
            .map(|x| (self, x).into())?)
    }
}
