use twilight_model::{
    guild::Member,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};

use crate::models::{role::Roles, Role};

impl super::MemberContext {
    /// Create this type from a few twilight datastructs
    pub fn twilight(
        twilight_member: Member,
        guild_id: Id<GuildMarker>,
        roles: Roles,
        everyone_role: Role,
        guild_owner_id: Id<UserMarker>,
    ) -> Self {
        Self {
            avatar: twilight_member.avatar,
            banner: None,
            boosting_since: twilight_member.premium_since,
            communication_disabled_until: twilight_member.communication_disabled_until,
            deafened: twilight_member.deaf,
            everyone_role,
            flags: twilight_member.flags,
            guild_id,
            guild_owner_id,
            joined_at: twilight_member.joined_at,
            muted: twilight_member.mute,
            nickname: twilight_member.nick,
            pending: twilight_member.pending,
            role_permissions: roles
                .iter()
                .map(|role| (role.role.id, role.role.permissions))
                .collect::<Vec<_>>(),
            roles,
            user: twilight_member.user.into(),
        }
    }
}
