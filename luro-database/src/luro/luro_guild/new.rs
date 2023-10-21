use std::sync::Arc;

use tracing::{error, warn};
use twilight_model::id::{marker::GuildMarker, Id};

use crate::{LuroDatabase, LuroGuild};

impl LuroGuild {
    pub async fn new(db: Arc<LuroDatabase>, guild_id: Id<GuildMarker>) -> anyhow::Result<Self> {
        match db.get_guild(&guild_id).await {
            Ok(Some(guild)) => Ok(guild.into()),
            Ok(None) => {
                warn!("Database failed to return a guild. Attempting to fetch and sync.");
                let twiligt_guild = db.twilight_client.guild(guild_id).await?.model().await?;
                match db.update_guild(twiligt_guild.clone()).await.map(|x| x.into()) {
                    Ok(guild) => Ok(guild),
                    Err(why) => {
                        error!(why = ?why, "Database errored while updating guild, falling back to client only");
                        Ok(twiligt_guild.into())
                    }
                }
            }
            Err(why) => {
                error!(why = ?why, "Database errored while fetching guild, falling back to client only");
                let twiligt_guild = db.twilight_client.guild(guild_id).await?.model().await?;
                Ok(twiligt_guild.into())
            }
        }
    }
}
