use luro_model::types::User;
use twilight_model::id::{Id, marker::{UserMarker, GuildMarker}};

use crate::Database;

impl Database {
    pub async fn member_fetch(&self, user_id: Id<UserMarker>, guild_id: Id<GuildMarker>) -> anyhow::Result<User> {
        if let Ok(Some(member)) = self.driver.get_member(user_id, guild_id).await {
            return Ok(member)
        }

        tracing::warn!("fetch_member - Failed to fetch member `{user_id}` of guild `{guild_id}`, falling back to Twilight");
        let member = self.api_client.guild_member(guild_id, user_id).await?.model().await?;
    
        for role_id in member.roles.clone() {
            if let Err(why) = self.driver.update_guild_member_roles(guild_id, role_id, user_id).await {
                tracing::error!(why = ?why, "fetch_member - failed to sync role `{role_id}` of member `{user_id}` of guild `{guild_id}` to the database!");
            }
        }
    
        if let Err(why) = self.driver.update_member((guild_id, &member)).await {
            tracing::error!(why = ?why, "fetch_member - failed to sync member `{user_id}` of guild `{guild_id}` to the database!");
        }
    
        Ok((member, guild_id).into())
    }
}