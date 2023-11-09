use luro_model::types::Role;
use twilight_model::id::{Id, marker::GuildMarker};

use crate::Database;

impl Database {
    pub async fn role_fetch_guild(&self, guild_id: Id<GuildMarker>) -> anyhow::Result<Vec<Role>> {
        if let Ok(roles) = self.driver.get_guild_roles(guild_id).await {
            if !roles.is_empty() {
                return Ok(roles)
            }
        }

        let twilight_roles = self.api_client.roles(guild_id).await?.model().await?;
        for role in &twilight_roles {
            if let Err(why) = self.driver.update_role((guild_id, role)).await {
                tracing::error!(why = ?why, "Failed to sync role to database")
            }
        }

        Ok(twilight_roles.into_iter().map(|role|(guild_id, role).into()).collect())
    }
}