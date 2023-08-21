use std::cmp::Ordering;

use twilight_cache_inmemory::GuildResource;
use twilight_model::{
    guild::Role,
    id::{marker::RoleMarker, Id}
};

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
    pub id: Id<RoleMarker>,
    pub position: i64,
    pub colour: u32
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
            position: role.position,
            colour: role.color
        }
    }
}

impl From<GuildResource<Role>> for RoleOrdering {
    fn from(role: GuildResource<Role>) -> Self {
        Self {
            id: role.id,
            position: role.position,
            colour: role.color
        }
    }
}
