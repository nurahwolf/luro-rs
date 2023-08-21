use serde::{Deserialize, Serialize};
use twilight_cache_inmemory::model::CachedMember;
use twilight_model::{
    guild::{Member, MemberFlags, PartialMember, Permissions},
    id::{
        marker::{RoleMarker, UserMarker},
        Id
    },
    util::{ImageHash, Timestamp}
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct LuroMember {
    /// User ID
    pub id: Option<Id<UserMarker>>,
    /// Member's guild avatar.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub avatar: Option<ImageHash>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub communication_disabled_until: Option<Timestamp>,
    #[serde(default)]
    pub deaf: bool,
    /// Flags for the member.
    ///
    /// Defaults to an empty bitfield.
    pub flags: MemberFlags,
    pub joined_at: Timestamp,
    #[serde(default)]
    pub mute: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub nick: Option<String>,
    /// Whether the user has yet to pass the guild's [Membership Screening]
    /// requirements.
    #[serde(default)]
    pub pending: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub premium_since: Option<Timestamp>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub role_ids: Vec<Id<RoleMarker>>,
    /// Permission data for the member.
    ///
    /// Sent in an [`Interaction`].
    ///
    /// [`Interaction`]: crate::application::interaction::Interaction
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub permissions: Option<Permissions>
}

impl LuroMember {
    /// Update this type from a user. Consider creating a default and then calling this function if you need a blank slate
    pub fn update_member(&mut self, member: &Member) -> &mut Self {
        self.avatar = member.avatar;
        self.communication_disabled_until = member.communication_disabled_until;
        self.deaf = member.deaf;
        self.flags = member.flags;
        self.id = Some(member.user.id);
        self.joined_at = member.joined_at;
        self.mute = member.mute;
        self.nick = member.nick.clone();
        self.pending = member.pending;
        self.premium_since = member.premium_since;
        self.role_ids = member.roles.clone();
        self
    }

    /// Update this type from a user. Consider creating a default and then calling this function if you need a blank slate
    pub fn update_partialmember(&mut self, member: &PartialMember) -> &mut Self {
        if let Some(user) = &member.user {
            self.id = Some(user.id);
        }
        self.avatar = member.avatar;
        self.communication_disabled_until = member.communication_disabled_until;
        self.deaf = member.deaf;
        self.flags = member.flags;
        self.joined_at = member.joined_at;
        self.mute = member.mute;
        self.nick = member.nick.clone();
        self.premium_since = member.premium_since;
        self.role_ids = member.roles.clone();
        self.permissions = member.permissions;
        self
    }
}

impl From<&PartialMember> for LuroMember {
    fn from(member: &PartialMember) -> Self {
        let id = member.user.as_ref().map(|user| user.id);
        Self {
            avatar: member.avatar,
            communication_disabled_until: member.communication_disabled_until,
            deaf: member.deaf,
            flags: member.flags,
            joined_at: member.joined_at,
            mute: member.mute,
            nick: member.nick.clone(),
            pending: false,
            premium_since: member.premium_since,
            role_ids: member.roles.clone(),
            permissions: member.permissions,
            id
        }
    }
}

impl From<&Member> for LuroMember {
    fn from(member: &Member) -> Self {
        Self {
            avatar: member.avatar,
            communication_disabled_until: member.communication_disabled_until,
            deaf: member.deaf,
            flags: member.flags,
            joined_at: member.joined_at,
            mute: member.mute,
            nick: member.nick.clone(),
            pending: member.pending,
            premium_since: member.premium_since,
            role_ids: member.roles.clone(),
            permissions: None,
            id: Some(member.user.id)
        }
    }
}

impl From<&CachedMember> for LuroMember {
    fn from(member: &CachedMember) -> Self {
        Self {
            id: Some(member.user_id()),
            avatar: member.avatar(),
            communication_disabled_until: member.communication_disabled_until(),
            deaf: member.deaf().unwrap_or_default(),
            flags: member.flags(),
            joined_at: member.joined_at(),
            mute: member.mute().unwrap_or_default(),
            nick: member.nick().map(|s| s.to_string()),
            pending: member.pending(),
            premium_since: member.premium_since(),
            role_ids: member.roles().to_vec(),
            permissions: Default::default()
        }
    }
}
