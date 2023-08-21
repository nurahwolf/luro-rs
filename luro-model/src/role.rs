use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::{Permissions, Role, RoleFlags, RoleTags},
    id::{marker::RoleMarker, Id},
    util::ImageHash
};

/// A [BTreeMap] of [LuroRole], keyed by a [RoleMarker]
pub type LuroRoles = BTreeMap<Id<RoleMarker>, LuroRole>;
/// A [BTreeMap] of [RoleMarker], keyed by [usize]
pub type LuroRolePositions = BTreeMap<usize, Id<RoleMarker>>;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LuroRole {
    #[serde(default)]
    pub color: u32,
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
    pub unicode_emoji: Option<String>
}

impl Default for LuroRole {
    fn default() -> Self {
        Self {
            color: Default::default(),
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
            unicode_emoji: Default::default()
        }
    }
}

impl From<Role> for LuroRole {
    fn from(role: Role) -> Self {
        Self {
            color: role.color,
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
            unicode_emoji: role.unicode_emoji
        }
    }
}
