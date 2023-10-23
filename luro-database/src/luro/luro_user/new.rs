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
            new_api(db.clone(), user_id, guild_id).await?; // If set to true, we call this to ensure that the user is updated in the database
        }

        match guild_id {
            Some(guild_id) => match db.get_member(user_id.get() as i64, guild_id.get() as i64).await {
                Ok(Some(member)) => Ok(member),
                Ok(None) => {
                    warn!("Database failed to return a user. Attempting to fetch and sync.");
                    new_api(db, user_id, Some(guild_id)).await
                }
                Err(why) => {
                    error!(why = ?why, "Database errored while fetching user, falling back to client only");
                    new_api(db, user_id, Some(guild_id)).await
                }
            },
            None => match db.get_user(&user_id).await {
                Ok(Some(user)) => Ok(user),
                Ok(None) => {
                    warn!("Database failed to return a user. Attempting to fetch and sync.");
                    new_api(db, user_id, guild_id).await
                }
                Err(why) => {
                    error!(why = ?why, "Database errored while fetching user, falling back to client only");
                    new_api(db, user_id, guild_id).await
                }
            },
        }
    }
}

/// Forces the type to be instanced from the Discord API, useful for when you want to know you has the most fresh data possible
async fn new_api(db: Arc<LuroDatabase>, user_id: Id<UserMarker>, guild_id: Option<Id<GuildMarker>>) -> anyhow::Result<LuroUser> {
    match guild_id {
        Some(guild_id) => {
            let twilight_member = db.twilight_client.guild_member(guild_id, user_id).await?.model().await?;
            match db.update_member((guild_id, twilight_member.clone())).await {
                Ok(_) => Ok((twilight_member, guild_id).into()),
                Err(why) => {
                    error!(why = ?why, "new_api - Database errored while updating member");
                    Ok((twilight_member, guild_id).into())
                }
            }
        }
        None => {
            let twilight_user = db.twilight_client.user(user_id).await?.model().await?;
            match db.update_user(twilight_user.clone()).await {
                Ok(_) => Ok(twilight_user.into()),
                Err(why) => {
                    error!(why = ?why, "new_api - Database errored while updating user");
                    Ok(twilight_user.into())
                }
            }
        }
    }
}
