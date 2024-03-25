use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::{database::Error, user::MemberContext};

impl crate::database::Database {
    // Fetch a member from the database. Note that due to the need to query the database twice, this does not get roles automatically.
    pub async fn fetch_member(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) -> Result<MemberContext, Error> {
        #[cfg(feature = "database-sqlx")]
        match self.sqlx_driver.fetch_member(guild_id, user_id).await {
            Ok(Some(user)) => return Ok(user),
            Ok(None) => tracing::warn!(
                "The member `{user_id}` in guild `{guild_id}` was requested from the database, but the member was not present."
            ),
            Err(why) => tracing::error!(
                ?why,
                "Database failed to fetch member `{user_id}` in guild `{guild_id}`, falling back to Twilight."
            ),
        };

        Ok(self
            .twilight_client
            .guild_member(guild_id, user_id)
            .await?
            .model()
            .await
            .map(|x| (guild_id, x).into())?)
    }
}
