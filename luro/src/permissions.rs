use std::cmp::Ordering;

use anyhow::anyhow;
use twilight_http::Client;
use twilight_model::{
    channel::ChannelType,
    guild::{Guild, Permissions, Role},
    id::{
        marker::{ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};
use twilight_util::permission_calculator::PermissionCalculator;

/// Calculate the permissions for a given guild.
pub struct GuildPermissions<'a> {
    twilight_client: &'a Client,
    guild: Guild,
}

impl<'a> GuildPermissions<'a> {
    /// Initialize [`GuildPermissions`] with from a guild.
    pub async fn new(
        twilight_client: &'a Client,
        guild_id: &Id<GuildMarker>,
    ) -> Result<GuildPermissions<'a>, anyhow::Error> {
        let guild = twilight_client.guild(*guild_id).await?.model().await?;

        Ok(Self {
            twilight_client,
            guild,
        })
    }

    /// Compute permissions for a given guild member.
    pub async fn member(
        &self,
        member_id: Id<UserMarker>,
        member_roles: &[Id<RoleMarker>],
    ) -> Result<LuroPermissions<'a>, anyhow::Error> {
        LuroPermissions::new(self, member_id, member_roles).await
    }

    /// Compute permissions for the current bot member.
    pub async fn current_member(&self) -> Result<LuroPermissions<'a>, anyhow::Error> {
        LuroPermissions::current_member(self).await
    }
}

/// Calculate the permissions of a member with information from the cache.
pub struct LuroPermissions<'a> {
    twilight_client: &'a Client,
    guild_id: Id<GuildMarker>,
    member_id: Id<UserMarker>,
    member_roles: MemberRoles,
    is_owner: bool,
}

impl<'a> LuroPermissions<'a> {
    /// Initialize [`GuildPermissions`] from a cache client.
    ///
    /// If the guild is not found in the cache, [`None`] is returned.
    pub async fn new(
        guild_permissions: &GuildPermissions<'a>,
        member_id: Id<UserMarker>,
        member_roles: &[Id<RoleMarker>],
    ) -> Result<LuroPermissions<'a>, anyhow::Error> {
        let guild_id = guild_permissions.guild.id;
        let is_owner = member_id == guild_permissions.guild.owner_id;

        let member_roles = MemberRoles::query(
            guild_permissions.twilight_client,
            guild_id,
            member_roles.iter(),
        )
        .await?;

        Ok(Self {
            twilight_client: guild_permissions.twilight_client,
            guild_id,
            member_id,
            member_roles,
            is_owner,
        })
    }

    /// Initialize [`LuroPermissions`] for the bot current member.
    pub async fn current_member(
        guild_permissions: &GuildPermissions<'a>,
    ) -> Result<LuroPermissions<'a>, anyhow::Error> {
        let member = guild_permissions
            .twilight_client
            .guild_member(
                guild_permissions.guild.id,
                guild_permissions
                    .twilight_client
                    .current_user()
                    .await?
                    .model()
                    .await?
                    .id,
            )
            .await?
            .model()
            .await?;

        let guild_id = guild_permissions.guild.id;
        let is_owner = member.user.id == guild_permissions.guild.owner_id;

        let member_roles = MemberRoles::query(
            guild_permissions.twilight_client,
            guild_id,
            member.roles.iter(),
        )
        .await?;

        Ok(Self {
            twilight_client: guild_permissions.twilight_client,
            guild_id,
            member_id: member.user.id,
            member_roles,
            is_owner,
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
            let mut roles: Vec<_> = self
                .member_roles
                .roles
                .iter()
                .map(RoleOrdering::from)
                .collect();
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

        let calculator =
            PermissionCalculator::new(self.guild_id, self.member_id, everyone_role, &member_roles);

        calculator.root()
    }

    /// Calculate the permissions of the user in a given channel.
    ///
    /// This method also return the [`ChannelType`] of the requested channel
    /// to handle the case where the channel is a thread.
    pub async fn channel(
        &self,
        channel: Id<ChannelMarker>,
    ) -> Result<(Permissions, ChannelType), anyhow::Error> {
        let mut channel = self.twilight_client.channel(channel).await?.model().await?;

        // If the channel is a thread, get the parent channel.
        if channel.kind.is_thread() {
            if let Some(parent_id) = channel.parent_id {
                channel = self
                    .twilight_client
                    .channel(parent_id)
                    .await?
                    .model()
                    .await?;
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

        let calculator =
            PermissionCalculator::new(self.guild_id, self.member_id, everyone_role, &member_roles);

        let kind = channel.kind;
        let permissions =
            calculator.in_channel(kind, &channel.permission_overwrites.unwrap_or_default());

        Ok((permissions, kind))
    }
}

/// List of resolved roles of a member.
struct MemberRoles {
    /// Everyone role
    pub everyone: Role,
    /// List of roles of the user
    pub roles: Vec<Role>,
}

impl MemberRoles {
    /// Query roles of a member in the cache.
    async fn query(
        twilight_client: &Client,
        guild_id: Id<GuildMarker>,
        member_roles: impl Iterator<Item = &Id<RoleMarker>>,
    ) -> Result<MemberRoles, anyhow::Error> {
        let everyone_id = guild_id.cast();
        let mut everyone_role = None;
        let mut roles = Vec::new();
        let guild = twilight_client.guild(guild_id).await?.model().await?;

        // Filter everyone role and other roles
        for member_role_id in member_roles {
            for role in &guild.roles {
                if role.id == everyone_id {
                    everyone_role = Some(role);
                } else if &role.id == member_role_id {
                    roles.push(role.clone())
                }
            }
        }

        if let Some(everyone) = everyone_role {
            Ok(MemberRoles {
                everyone: everyone.clone(),
                roles,
            })
        } else {
            Err(anyhow!("everyone role not found in cache"))
        }
    }
}

/// Compares the position of two roles.
///
/// This type is used to compare positions of two different roles, using the
/// [`Ord`] trait.
///
/// According to [twilight-model documentation]:
///
/// > Roles are primarily ordered by their position in descending order.
/// > For example, a role with a position of 17 is considered a higher role than
/// > one with a position of 12.
/// >
/// > Discord does not guarantee that role positions are positive, unique, or
/// > contiguous. When two or more roles have the same position then the order
/// > is based on the roles’ IDs in ascending order. For example, given two roles
/// > with positions of 10 then a role with an ID of 1 would be considered a
/// > higher role than one with an ID of 20.
///
/// [twilight-model documentation]: https://docs.rs/twilight-model/0.10.2/twilight_model/guild/struct.Role.html#impl-Ord
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoleOrdering {
    id: Id<RoleMarker>,
    position: i64,
}

impl Ord for RoleOrdering {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position
            .cmp(&other.position)
            .then(self.id.get().cmp(&other.id.get()))
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
            position: role.position,
        }
    }
}
