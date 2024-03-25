use twilight_model::id::{marker::GuildMarker, Id};

use crate::{
    database::sqlx::{Database, Error},
    guild::Guild,
};

impl Database {
    pub async fn fetch_guild(&self, guild_id: Id<GuildMarker>) -> Result<Guild, Error> {
        // match fetch_user(self, user_id).await {
        //     Ok(Some(user)) => return Ok(user),
        //     Ok(None) => tracing::warn!("The user `{user_id}` was requested from the database, but the user was not present."),
        //     Err(why) => tracing::error!(?why, "Database failed to fetch user `{user_id}`, falling back to Twilight."),
        // };

        Ok(self.twilight_driver.fetch_guild(guild_id).await?)
    }
}
