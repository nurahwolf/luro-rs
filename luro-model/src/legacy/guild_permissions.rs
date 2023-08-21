use twilight_http::Client;
use twilight_model::{
    guild::Guild,
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id
    }
};

use super::luro_permissions::LuroPermissions;

/// Calculate the permissions for a given guild.
pub struct GuildPermissions<'a> {
    pub twilight_client: &'a Client,
    pub guild: Guild
}

impl<'a> GuildPermissions<'a> {
    /// Initialize [`GuildPermissions`] with from a guild.
    pub async fn new(twilight_client: &'a Client, guild_id: &Id<GuildMarker>) -> Result<GuildPermissions<'a>, anyhow::Error> {
        let guild = twilight_client.guild(*guild_id).await?.model().await?;

        Ok(Self { twilight_client, guild })
    }

    /// Compute permissions for a given guild member.
    pub async fn member(
        &self,
        member_id: Id<UserMarker>,
        member_roles: &[Id<RoleMarker>]
    ) -> Result<LuroPermissions<'a>, anyhow::Error> {
        LuroPermissions::new(self, member_id, member_roles).await
    }

    /// Compute permissions for the current bot member.
    pub async fn current_member(&self) -> Result<LuroPermissions<'a>, anyhow::Error> {
        LuroPermissions::current_member(self).await
    }
}
