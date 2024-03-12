use twilight_model::guild::Permissions;

use crate::models::Role;

impl super::MemberContext {
    pub fn permission_matrix(&self) -> (Option<Role>, Permissions) {
        let mut roles = self.roles.clone();
        roles.sort();

        (roles.first().cloned(), self.permission_calculator().root())
    }
}
