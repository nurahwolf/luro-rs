use twilight_model::{
    channel::ChannelType,
    guild::Permissions,
    id::{
        marker::{ChannelMarker, RoleMarker, UserMarker},
        Id
    }
};
use twilight_util::permission_calculator::PermissionCalculator;

use super::{GuildPermissions, LuroPermissions, MemberRoles, RoleOrdering};

impl<'a> LuroPermissions<'a> {
    /// Initialize [`GuildPermissions`] from a cache client.
    ///
    /// If the guild is not found in the cache, [`None`] is returned.
    pub async fn new(
        guild_permissions: &GuildPermissions<'a>,
        member_id: Id<UserMarker>,
        member_roles: &[Id<RoleMarker>]
    ) -> anyhow::Result<LuroPermissions<'a>> {
        let guild_id = guild_permissions.guild.id;
        let is_owner = member_id == guild_permissions.guild.owner_id;

        let member_roles = MemberRoles::query(guild_permissions.twilight_client, guild_id, member_roles.iter()).await?;

        Ok(Self {
            twilight_client: guild_permissions.twilight_client,
            guild_id,
            member_id,
            member_roles,
            is_owner
        })
    }

    /// Initialize [`LuroPermissions`] for the bot current member.
    pub async fn current_member(guild_permissions: &GuildPermissions<'a>) -> anyhow::Result<LuroPermissions<'a>> {
        let member = guild_permissions
            .twilight_client
            .guild_member(
                guild_permissions.guild.id,
                guild_permissions.twilight_client.current_user().await?.model().await?.id
            )
            .await?
            .model()
            .await?;

        let guild_id = guild_permissions.guild.id;
        let is_owner = member.user.id == guild_permissions.guild.owner_id;

        let member_roles = MemberRoles::query(guild_permissions.twilight_client, guild_id, member.roles.iter()).await?;

        Ok(Self {
            twilight_client: guild_permissions.twilight_client,
            guild_id,
            member_id: member.user.id,
            member_roles,
            is_owner
        })
    }

    /// Checks if a user is the owner of a guild.
    pub fn is_owner(&self) -> bool {
        self.is_owner
    }

    /// Returns the highest role of a user.
    pub fn highest_role(&self) -> RoleOrdering {
        if self.member_roles.roles.is_empty() {
            RoleOrdering::from(&self.member_roles.everyone)
        } else {
            let mut roles: Vec<_> = self.member_roles.roles.iter().map(RoleOrdering::from).collect();
            roles.sort();

            *roles.last().unwrap() // SAFETY: roles is not empty
        }
    }

    /// Calculate the permissions of the user in the guild.
    pub fn guild(&self) -> Permissions {
        // Owners have all permissions
        if self.is_owner {
            return Permissions::all();
        }

        // TODO: extract this into a function
        let everyone_role = self.member_roles.everyone.permissions;
        let member_roles = self
            .member_roles
            .roles
            .iter()
            .map(|role| (role.id, role.permissions))
            .collect::<Vec<_>>();

        let calculator = PermissionCalculator::new(self.guild_id, self.member_id, everyone_role, &member_roles);

        calculator.root()
    }

    /// Calculate the permissions of the user in a given channel.
    ///
    /// This method also return the [`ChannelType`] of the requested channel
    /// to handle the case where the channel is a thread.
    pub async fn channel(&self, channel: Id<ChannelMarker>) -> Result<(Permissions, ChannelType), anyhow::Error> {
        let mut channel = self.twilight_client.channel(channel).await?.model().await?;

        // If the channel is a thread, get the parent channel.
        if channel.kind.is_thread() {
            if let Some(parent_id) = channel.parent_id {
                channel = self.twilight_client.channel(parent_id).await?.model().await?;
            }
        }

        // TODO: extract this into a function
        let everyone_role = self.member_roles.everyone.permissions;
        let member_roles = self
            .member_roles
            .roles
            .iter()
            .map(|role| (role.id, role.permissions))
            .collect::<Vec<_>>();

        let calculator = PermissionCalculator::new(self.guild_id, self.member_id, everyone_role, &member_roles);

        let kind = channel.kind;
        let permissions = calculator.in_channel(kind, &channel.permission_overwrites.unwrap_or_default());

        Ok((permissions, kind))
    }
}
