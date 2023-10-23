use std::sync::Arc;

use anyhow::Context;
use tracing::{error, warn};
use twilight_model::id::{
    marker::{GuildMarker, RoleMarker},
    Id,
};

use crate::{LuroDatabase, LuroRole};

impl LuroRole {
    pub async fn new(db: Arc<LuroDatabase>, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) -> anyhow::Result<Self> {
        // TODO: Return from twilight if the database failed
        match db.get_role(&guild_id, &role_id).await {
            Ok(Some(role)) => Ok(role),
            Ok(None) => {
                warn!("Failed to get role from database, falling back to twlight client");
                for role in db.twilight_client.roles(guild_id).await?.model().await? {
                    db.update_role((guild_id, role)).await?;
                }
                Ok(db.get_role(&guild_id, &role_id).await?.context("Expected to get updated role")?)
            }
            Err(why) => {
                error!(why = ?why, "Got an error while trying to fetch role from database");
                for role in db.twilight_client.roles(guild_id).await?.model().await? {
                    db.update_role((guild_id, role)).await?;
                }
                Ok(db.get_role(&guild_id, &role_id).await?.context("Expected to get updated role")?)
            }
        }
    }
}
