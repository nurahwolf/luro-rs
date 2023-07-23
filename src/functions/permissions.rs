use std::cmp::Ordering;

use anyhow::anyhow;
use tracing::debug;
use twilight_http::Client;
use twilight_model::{
    channel::ChannelType,
    guild::{Permissions, Role},
    id::{
        marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id
    }
};
use twilight_util::permission_calculator::PermissionCalculator;

use super::{GuildPermissions, LuroPermissions, MemberRoles, RoleOrdering};

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

impl MemberRoles {
    /// Query roles of a member in the cache.
    async fn query(
        twilight_client: &Client,
        guild_id: Id<GuildMarker>,
        member_roles: impl Iterator<Item = &Id<RoleMarker>>
    ) -> anyhow::Result<MemberRoles> {
        let mut everyone_role = None;
        let mut roles = Vec::new();
        let guild = twilight_client.guild(guild_id).await?.model().await?;
        let mut member_roles = member_roles.peekable();

        // In case the user has no roles, just return the everyone role. If there are roles, iterate through them
        if member_roles.peek().is_some() {
            // Filter everyone role and other roles
            for member_role_id in member_roles {
                debug!("User has roles");
                debug!("{member_role_id}");
                for role in &guild.roles {
                    // info!(role.name);

                    if role.id == guild_id.cast() {
                        everyone_role = Some(role);
                    } else if &role.id == member_role_id {
                        roles.push(role.clone())
                    }
                }
            }
        } else {
            debug!("User does not have roles, iterating through the guild_roles to grab the everyone role");
            for role in &guild.roles {
                if role.id == guild_id.cast() {
                    everyone_role = Some(role);
                }
            }
        }

        if let Some(everyone) = everyone_role {
            Ok(MemberRoles {
                everyone: everyone.clone(),
                roles
            })
        } else {
            Err(anyhow!("everyone role not found in cache"))
        }
    }
}

impl Ord for RoleOrdering {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position).then(self.id.get().cmp(&other.id.get()))
    }
}

impl PartialOrd for RoleOrdering {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&Role> for RoleOrdering {
    fn from(role: &Role) -> Self {
        Self {
            id: role.id,
            position: role.position
        }
    }
}
