mod new;

use std::cmp::Ordering;

use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::{Permissions, Role, RoleFlags, RoleTags},
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
    util::ImageHash,
};

use super::luro_role_data::LuroRoleData;

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LuroRole {
    /// Data that is only present if received from the database
    pub data: Option<LuroRoleData>,
    pub colour: u32,
    pub hoist: bool,
    /// Icon image hash.
    ///
    /// Present if the guild has the `ROLE_ICONS` feature and if the role has
    /// one.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,
    pub role_id: Id<RoleMarker>,
    pub guild_id: Id<GuildMarker>,
    pub managed: bool,
    pub mentionable: bool,
    pub name: String,
    pub permissions: Permissions,
    pub position: i64,
    /// Flags for this role.
    pub flags: RoleFlags,
    /// Tags about the role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<RoleTags>,
    /// Icon unicode emoji.
    ///
    /// Present if the guild has the `ROLE_ICONS` feature and if the role has
    /// one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<String>,
}

impl std::fmt::Display for LuroRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.role_id)
    }
}

impl Ord for LuroRole {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position
            .cmp(&other.position)
            .then(self.role_id.get().cmp(&other.role_id.get()))
            .reverse()
    }
}

impl PartialOrd for LuroRole {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<LuroRole> for Role {
    fn from(role: LuroRole) -> Self {
        Self {
            color: role.colour,
            hoist: role.hoist,
            icon: role.icon,
            id: role.role_id,
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

impl From<(Id<GuildMarker>, Role)> for LuroRole {
    fn from((guild_id, role): (Id<GuildMarker>, Role)) -> Self {
        Self {
            data: None,
            colour: role.color,
            hoist: role.hoist,
            icon: role.icon,
            role_id: role.id,
            guild_id,
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
