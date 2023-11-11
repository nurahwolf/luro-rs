use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::Permissions,
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};
use twilight_util::permission_calculator::PermissionCalculator;

use super::Role;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MemberData {
    #[serde(skip)]
    pub left_at: Option<time::OffsetDateTime>,
    pub roles: HashMap<Id<RoleMarker>, Role>,
    pub guild_id: Id<GuildMarker>,
    pub guild_owner_id: Id<UserMarker>,
    pub guild_everyone_role_permissions: Permissions,
    pub user_id: Id<UserMarker>,
    pub guild_owner: bool,
}

impl MemberData {
    /// Returns a vector of roles, sorted by the hiararchy
    pub fn sorted_roles(&self) -> Vec<&Role> {
        let mut roles = self.roles.values().collect::<Vec<_>>();
        roles.sort();
        roles
    }

    /// Returns the user's highest role that has a colour set. Returns none if there are no roles / no colours
    pub fn highest_role(&self) -> Option<&Role> {
        self.sorted_roles().first().cloned()
    }

    /// Returns the user's highest role that has a colour set. Returns none if there are no roles / no colours
    pub fn highest_role_colour(&self) -> Option<&Role> {
        self.sorted_roles().into_iter().find(|&role| role.colour != 0)
    }

    /// Fetches the member's permission calculator
    pub fn permission_calculator<'a>(&'a self, member_roles: &'a [(Id<RoleMarker>, Permissions)]) -> PermissionCalculator {
        PermissionCalculator::new(self.guild_id, self.user_id, self.guild_everyone_role_permissions, member_roles)
            .owner_id(self.guild_owner_id)
    }

    /// Gets all roles and their permissions, excluding the everyone role
    pub fn role_permissions(&self) -> Vec<(Id<RoleMarker>, Permissions)> {
        self.roles
            .iter()
            .filter(|(role_id, _)| role_id != &&self.guild_id.cast())
            .map(|(_, role)| (role.role_id, role.permissions))
            .collect()
    }
}
