mod new;

use serde::{Deserialize, Serialize};
use twilight_model::{
    guild::{Permissions, Role, RoleFlags, RoleTags},
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
    util::ImageHash,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuroRole {
    pub deleted: bool,
    pub guild_id: Id<GuildMarker>,
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
    pub icon: Option<String>,
    pub id: Id<RoleMarker>,
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

impl From<LuroRole> for Role {
    fn from(role: LuroRole) -> Self {
        Self {
            color: role.colour,
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

impl std::fmt::Display for LuroRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
