use std::sync::Arc;

use tracing::{error, warn};
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::{LuroDatabase, LuroUser};

impl LuroUser {
    pub async fn new(db: Arc<LuroDatabase>, user_id: Id<UserMarker>, guild_id: Option<Id<GuildMarker>>) -> anyhow::Result<Self> {
        match guild_id {
            Some(guild_id) => match db.get_member(&user_id, &guild_id).await {
                Ok(Some(member)) => Ok(member.into()),
                Ok(None) => {
                    warn!("Database failed to return a user. Attempting to fetch and sync.");
                    let twilight_member = db.twilight_client.guild_member(guild_id, user_id).await?.model().await?;
                    match db.update_member((guild_id, twilight_member.clone())).await {
                        Ok(user) => Ok(user.context("Should have got user back from database")?.into()),
                        Err(why) => {
                            error!(why = ?why, "Database errored while updating user, falling back to client only");
                            Ok((twilight_member, guild_id).into())
                        }
                    }
                }
                Err(why) => {
                    error!(why = ?why, "Database errored while fetching user, falling back to client only");
                    let twilight_member = db.twilight_client.guild_member(guild_id, user_id).await?.model().await?;
                    Ok((twilight_member, guild_id).into())
                }
            },
            None => match db.get_user(&user_id).await {
                Ok(Some(user)) => Ok(user.into()),
                Ok(None) => {
                    warn!("Database failed to return a user. Attempting to fetch and sync.");
                    let twilight_user = db.twilight_client.user(user_id).await?.model().await?;
                    match db.update_user(twilight_user.clone()).await.map(|x| x.into()) {
                        Ok(user) => Ok(user),
                        Err(why) => {
                            error!(why = ?why, "Database errored while updating user, falling back to client only");
                            Ok(twilight_user.into())
                        }
                    }
                }
                Err(why) => {
                    error!(why = ?why, "Database errored while fetching user, falling back to client only");
                    let twilight_user = db.twilight_client.user(user_id).await?.model().await?;
                    Ok(twilight_user.into())
                }
            },
        }
    }
}
