use std::sync::Arc;

use tracing::{error, warn};
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::{LuroDatabase, LuroUser};

impl LuroUser {
    /// Create a new type, attempting to get from the database first before falling back to using the discord API
    pub async fn new(
        db: Arc<LuroDatabase>,
        user_id: Id<UserMarker>,
        guild_id: Option<Id<GuildMarker>>,
        new_data: bool,
    ) -> anyhow::Result<Self> {
        if new_data {
            new_from_api(db.clone(), user_id, guild_id).await?;
        }

        // First attempt to map to a member
        if let Some(guild_id) = guild_id {
            if let Ok(member) = db.get_member(user_id.get() as i64, guild_id.get() as i64).await {
                return Ok(member);
            }

            warn!("Database failed to return a user. Attempting to fetch and sync.");
            return new_from_api(db, user_id, Some(guild_id)).await
        }

        if let Ok(Some(user)) = db.get_user(&user_id).await {
            return Ok(user);
        }

        warn!("Database failed to return a user. Attempting to fetch and sync.");
        new_from_api(db, user_id, guild_id).await
    }
}

/// Forces the type to be instanced from the Discord API, useful for when you want to know you has the most fresh data possible
async fn new_from_api(db: Arc<LuroDatabase>, user_id: Id<UserMarker>, guild_id: Option<Id<GuildMarker>>) -> anyhow::Result<LuroUser> {
    // First attempt to map to a member
    if let Some(guild_id) = guild_id {
        if let Ok(member) = db.twilight_client.guild_member(guild_id, user_id).await {
            let member = member.model().await?;

            if let Err(why) = db.update_member((guild_id, member.clone())).await {
                error!(why = ?why, "new_api - Database errored while updating member")
            }

            return Ok((member, guild_id).into());
        }
    }

    // Can't map the user to a member, so return a user
    let user = db.twilight_client.user(user_id).await?.model().await?;

    if let Err(why) = db.update_user(user.clone()).await {
        error!(why = ?why, "new_api - Database errored while updating user");
    }

    Ok(user.into())
}
