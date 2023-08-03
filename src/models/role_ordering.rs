use std::cmp::Ordering;

use twilight_model::guild::Role;

use super::RoleOrdering;

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
            position: role.position,
            colour: role.color
        }
    }
}
