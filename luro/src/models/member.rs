mod avatar_url;
mod highest_role;
mod permission_calculator;
mod permission_matrix;
mod twilight;
mod user_id;
mod username;

use twilight_model::{
    guild::Permissions,
    id::{
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
    util::{ImageHash, Timestamp},
};

use super::Role;

/// A warpper around [User], with [Member] details if [Id<GuildMarker>] was present on type creation.
/// Details are primarily fetched from the database, but this type can be instanced from a [User] / [Member] if that fails.
/// Also holds some additional which are relevent to Luro only. These are empty if the type was not instanced from the database.
#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct MemberContext {
    pub user: super::UserContext,
    pub avatar: Option<ImageHash>,
    pub banner: Option<ImageHash>,
    pub boosting_since: Option<Timestamp>,
    pub communication_disabled_until: Option<Timestamp>,
    pub deafened: bool,
    pub flags: twilight_model::guild::MemberFlags,
    pub joined_at: Option<twilight_model::util::Timestamp>,
    pub muted: bool,
    pub nickname: Option<String>,
    pub pending: bool,
    pub guild_id: Id<GuildMarker>,
    pub guild_owner_id: Id<UserMarker>,
    pub roles: Vec<Role>,
    pub everyone_role: Role,
    pub role_permissions: Vec<(Id<RoleMarker>, Permissions)>,
}

impl From<super::MemberContext> for twilight_model::guild::Member {
    fn from(member: super::MemberContext) -> Self {
        twilight_model::guild::Member {
            avatar: member.avatar,
            communication_disabled_until: member.communication_disabled_until,
            deaf: member.deafened,
            flags: member.flags,
            joined_at: member.joined_at,
            mute: member.muted,
            nick: member.nickname,
            pending: member.pending,
            premium_since: member.boosting_since,
            roles: member
                .roles
                .into_iter()
                .map(|x| x.role.id)
                .collect::<Vec<_>>(),
            user: member.user.into(),
        }
    }
}
