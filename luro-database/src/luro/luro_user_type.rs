use std::sync::Arc;

use sqlx::types::Json;
use twilight_model::{
    guild::Member,
    id::{marker::GuildMarker, Id},
    user::User,
};

use crate::{DatabaseUser, DbMember, LuroDatabase, LuroMember, LuroUserData};

use super::luro_user::LuroUser;

/// An Enum used to tell how a [LuroUser] was created. Additionally it wraps the type that created it.
///
/// There is also an implementation for turning this type into a [LuroUser]!
#[derive(Clone, Debug)]
pub enum LuroUserType {
    /// A type instanced from a Twilight [User]
    User(Arc<LuroDatabase>, User),
    /// A type instanced from a Twilight [Member]
    Member(Arc<LuroDatabase>, Member, Id<GuildMarker>),
    /// A type instanced from our database
    DbUser(Arc<LuroDatabase>, DatabaseUser),
    /// A type instanced from our database, with guild information available
    DbMember(Arc<LuroDatabase>, DatabaseUser, DbMember),
}

impl From<LuroUserType> for LuroUser {
    fn from(instance: LuroUserType) -> Self {
        match instance.clone() {
            LuroUserType::User(db, user) => Self {
                instance,
                db,
                member: None,
                data: None,
                accent_colour: user.accent_color.map(|x| x as i32),
                avatar_decoration: user.avatar_decoration.map(Json),
                avatar: user.avatar.map(Json),
                banner: user.banner.map(Json),
                bot: user.bot,
                discriminator: user.discriminator as i16,
                email: user.email,
                flags: user.flags.map(Json),
                global_name: user.global_name,
                locale: user.locale,
                mfa_enabled: user.mfa_enabled,
                name: user.name,
                premium_type: user.premium_type.map(Json),
                public_flags: user.public_flags.map(Json),
                system: user.system,
                verified: user.verified,
                user_id: user.id.get() as i64,
            },
            LuroUserType::DbUser(db, user) => Self {
                instance,
                db,
                data: Some(LuroUserData {
                    permissions: user.user_permissions,
                }),
                member: None,
                accent_colour: user.accent_colour,
                avatar_decoration: user.avatar_decoration,
                avatar: user.avatar,
                banner: user.banner,
                bot: user.bot,
                discriminator: user.discriminator,
                email: user.email,
                flags: user.flags,
                global_name: user.global_name,
                locale: user.locale,
                mfa_enabled: user.mfa_enabled,
                name: user.name,
                premium_type: user.premium_type,
                public_flags: user.public_flags,
                system: user.system,
                verified: user.verified,
                user_id: user.user_id,
            },
            LuroUserType::Member(db, member, guild_id) => Self {
                instance,
                db,
                data: None,
                member: Some((guild_id, member.clone()).into()),
                accent_colour: member.user.accent_color.map(|x| x as i32),
                avatar_decoration: member.user.avatar_decoration.map(Json),
                avatar: member.user.avatar.map(Json),
                banner: member.user.banner.map(Json),
                bot: member.user.bot,
                discriminator: member.user.discriminator as i16,
                email: member.user.email,
                flags: member.user.flags.map(Json),
                global_name: member.user.global_name,
                locale: member.user.locale,
                mfa_enabled: member.user.mfa_enabled,
                name: member.user.name.clone(),
                premium_type: member.user.premium_type.map(Json),
                public_flags: member.user.public_flags.map(Json),
                system: member.user.system,
                verified: member.user.verified,
                user_id: member.user.id.get() as i64,
            },
            LuroUserType::DbMember(db, user, member) => Self {
                instance,
                db,
                data: Some(LuroUserData {
                    permissions: user.user_permissions,
                }),
                member: Some(LuroMember {
                    user_id: user.user_id,
                    guild_id: member.guild_id,
                    avatar: member.avatar,
                    boosting_since: member.boosting_since,
                    communication_disabled_until: member.communication_disabled_until,
                    deafened: member.deafened,
                    flags: member.flags,
                    muted: member.muted,
                    nickname: member.nickname,
                    pending: member.pending,
                }),
                accent_colour: user.accent_colour,
                avatar_decoration: user.avatar_decoration,
                avatar: user.avatar,
                banner: user.banner,
                bot: user.bot,
                discriminator: user.discriminator,
                email: user.email,
                flags: user.flags,
                global_name: user.global_name,
                locale: user.locale,
                mfa_enabled: user.mfa_enabled,
                name: user.name,
                premium_type: user.premium_type,
                public_flags: user.public_flags,
                system: user.system,
                verified: user.verified,
                user_id: user.user_id,
            },
        }
    }
}
