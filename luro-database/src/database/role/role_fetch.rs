use anyhow::Context;
use luro_model::types::Role;
use twilight_model::id::{Id, marker::{GuildMarker, RoleMarker}};

use crate::Database;

impl Database {
    pub async fn role_fetch(&self, guild_id: Id<GuildMarker>, role_id: Id<RoleMarker>) -> anyhow::Result<Role> {
        if let Ok(Some(role)) = self.driver.role_fetch(guild_id, role_id).await {
            return Ok(role)
        }

        tracing::warn!("Failed to get role from database, falling back to twlight client");
        let mut requested_role = None;
        for role in self.api_client.roles(guild_id).await?.model().await? {
            self.driver.update_role((guild_id, &role)).await?;
            if role.id == role_id {
                requested_role = Some((guild_id, role).into());
            }
        }

        requested_role.context("Role could not be found")
    }
}