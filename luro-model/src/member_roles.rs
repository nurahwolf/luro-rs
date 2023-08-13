use anyhow::anyhow;
use tracing::debug;
use twilight_http::Client;
use twilight_model::{
    guild::Role,
    id::{
        marker::{GuildMarker, RoleMarker},
        Id
    }
};

/// List of resolved roles of a member.
pub struct MemberRoles {
    /// Everyone role
    pub everyone: Role,
    /// List of roles of the user
    pub roles: Vec<Role>
}

impl MemberRoles {
    /// Query roles of a member in the cache.
    pub async fn query(
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
