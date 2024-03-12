
use twilight_model::id::{
    marker::GuildMarker,
    Id,
};

pub type Roles = Vec<Role>;

/// Note that it is possible to compare the positions between roles, using the [`Ord`] trait.
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
#[derive(Clone, Debug, serde::Deserialize, PartialEq, Eq, serde::Serialize)]
pub struct Role {
    pub role: twilight_model::guild::Role,
    pub deleted: Option<bool>,
    pub guild_id: Id<GuildMarker>,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.role.id)
    }
}

impl Ord for Role {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.role
            .position
            .cmp(&other.role.position)
            .then(self.role.id.get().cmp(&other.role.id.get()))
            .reverse()
    }
}

impl PartialOrd for Role {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Role> for twilight_model::guild::Role {
    fn from(role: Role) -> Self {
        role.role
    }
}

impl From<(Id<GuildMarker>, twilight_model::guild::Role)> for Role {
    fn from((guild_id, role): (Id<GuildMarker>, twilight_model::guild::Role)) -> Self {
        Self {
            deleted: None,
            guild_id,
            role,
        }
    }
}
