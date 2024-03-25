use twilight_util::permission_calculator::PermissionCalculator;

impl super::MemberContext {
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
