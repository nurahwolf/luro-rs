use twilight_util::permission_calculator::PermissionCalculator;

impl super::MemberContext {
    pub fn permission_calculator(&self) -> PermissionCalculator {
        let guild_id = self.guild_id;
        let user_id = self.user.user_id;
        let everyone_role = self.everyone_role.role.permissions;
        let member_roles = &self.role_permissions;
        PermissionCalculator::new(guild_id, user_id, everyone_role, &member_roles)
            .owner_id(self.guild_owner_id)
    }

    // /// Gets all roles and their permissions, excluding the everyone role
    // pub fn role_permissions(
    //     &self,
    // ) -> Vec<(
    //     Id<twilight_model::id::marker::RoleMarker>,
    //     twilight_model::guild::Permissions,
    // )> {
    //     self.roles
    //         .iter()
    //         .filter(|(role_id, _)| role_id != &&self.guild_id.cast())
    //         .map(|(_, role)| (role.role.id, role.role.permissions))
    //         .collect()
    // }
}
