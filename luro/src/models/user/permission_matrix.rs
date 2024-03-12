use twilight_model::guild::Permissions;

use crate::models::Role;

impl super::User {
    pub fn permission_matrix(&self) -> Option<(Option<Role>, Permissions)> {
        match self {
            super::User::User(_) => None,
            super::User::Member(member) => Some(member.permission_matrix()),
        }
    }
}
