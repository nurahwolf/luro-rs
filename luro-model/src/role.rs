use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
};

use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::{Permissions, Role, RoleFlags, RoleTags},
    id::{marker::RoleMarker, Id},
    util::ImageHash,
};

/// A [HashMap] of [LuroRole], keyed by a [RoleMarker]
pub type LuroRoles = HashMap<Id<RoleMarker>, LuroRole>;
/// A [BTreeMap] of [RoleMarker], keyed by [usize]
pub type LuroRolePositions = BTreeMap<usize, Id<RoleMarker>>;

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
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LuroRole {
    #[serde(default)]
    pub colour: u32,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub hoist: bool,
    /// Icon image hash.
    ///
    /// Present if the guild has the `ROLE_ICONS` feature and if the role has
    /// one.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub icon: Option<ImageHash>,
    pub id: Id<RoleMarker>,
    #[serde(default)]
    pub managed: bool,
    #[serde(default)]
    pub mentionable: bool,
    #[serde(default)]
    pub name: String,
    pub permissions: Permissions,
    #[serde(default)]
    pub position: i64,
    /// Flags for this role.
    pub flags: RoleFlags,
    /// Tags about the role.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<RoleTags>,
    /// Icon unicode emoji.
    ///
    /// Present if the guild has the `ROLE_ICONS` feature and if the role has
    /// one.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub unicode_emoji: Option<String>,
}

impl LuroRole {
    pub fn role_permission(&self) -> (Id<RoleMarker>, Permissions) {
        (self.id, self.permissions)
    }

    pub fn role_ids(roles: Vec<LuroRole>) -> Vec<Id<RoleMarker>> {
        roles.into_iter().map(|x| x.id).collect()
    }
}

impl Ord for LuroRole {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position
            .cmp(&other.position)
            .then(self.id.get().cmp(&other.id.get()))
            .reverse()
    }
}

impl PartialOrd for LuroRole {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for LuroRole {
    fn default() -> Self {
        Self {
            colour: Default::default(),
            deleted: Default::default(),
            hoist: Default::default(),
            icon: Default::default(),
            id: Id::new(0),
            managed: Default::default(),
            mentionable: Default::default(),
            name: Default::default(),
            permissions: Permissions::empty(),
            position: Default::default(),
            flags: RoleFlags::empty(),
            tags: Default::default(),
            unicode_emoji: Default::default(),
        }
    }
}

impl From<Role> for LuroRole {
    fn from(role: Role) -> Self {
        Self {
            colour: role.color,
            deleted: false,
            hoist: role.hoist,
            icon: role.icon,
            id: role.id,
            managed: role.managed,
            mentionable: role.mentionable,
            name: role.name,
            permissions: role.permissions,
            position: role.position,
            flags: role.flags,
            tags: role.tags,
            unicode_emoji: role.unicode_emoji,
        }
    }
}

impl From<Id<RoleMarker>> for LuroRole {
    fn from(id: Id<RoleMarker>) -> Self {
        LuroRole {
            id,
            ..Default::default()
        }
    }
}

impl From<&Id<RoleMarker>> for LuroRole {
    fn from(id: &Id<RoleMarker>) -> Self {
        LuroRole {
            id: *id,
            ..Default::default()
        }
    }
}
