use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};
use twilight_model::{id::{marker::{RoleMarker, GuildMarker, UserMarker}, Id}, guild::Permissions};
use twilight_util::permission_calculator::PermissionCalculator;

use crate::{LuroRole, LuroDatabase, LuroGuild};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LuroMemberData {
    #[serde(skip)]
    pub left_at: Option<time::OffsetDateTime>,
    pub roles: HashMap<Id<RoleMarker>, LuroRole>,
    pub guild_id: Id<GuildMarker>,
    pub user_id: Id<UserMarker>,
    pub guild_owner: bool,
}

impl LuroMemberData {
    /// Returns a vector of roles, sorted by the hiararchy
    pub fn sorted_roles(&self) -> Vec<&LuroRole> {
        let mut roles = self.roles.values().collect::<Vec<_>>();
        roles.sort();
        roles
    }

    /// Fetches the member's permission calculator
    ///
    /// TODO: Remove the member_roles parameter
    pub async fn permission_calculator<'a>(
        &'a self,
        db: Arc<LuroDatabase>,
        member_roles: &'a [(Id<RoleMarker>, Permissions)],
    ) -> anyhow::Result<PermissionCalculator> {
        let guild = LuroGuild::new(db.clone(), self.guild_id).await?;
        let everyone_role = guild.get_everyone_role(db).await?;

        Ok(PermissionCalculator::new(self.guild_id, self.user_id, everyone_role.permissions, member_roles).owner_id(guild.owner_id))
    }

    /// Gets all roles and their permissions, excluding the everyone role
    pub fn role_permissions(&self) -> Vec<(Id<RoleMarker>, Permissions)> {
        let mut new_roles = self.roles.clone();
        new_roles.retain(|_, role| role.role_id != self.guild_id.cast());
        new_roles.values().map(|x| (x.role_id, x.permissions)).collect()
    }
}
