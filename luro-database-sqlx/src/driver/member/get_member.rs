use std::collections::HashMap;

use futures_util::TryStreamExt;
use luro_model::types::{Member, MemberData, Role, RoleData, User, UserData};
use sqlx::types::Json;
use twilight_model::guild::{MemberFlags, RoleTags};
use twilight_model::id::marker::{GuildMarker, UserMarker};

use twilight_model::user::{PremiumType, UserFlags};
use twilight_model::util::{ImageHash, Timestamp};
use twilight_model::{
    guild::{Permissions, RoleFlags},
    id::Id,
};

use crate::types::{DbGender, DbSexuality, DbUserPermissions};
use crate::SQLxDriver;

impl SQLxDriver {
    pub async fn get_member(&self, user_id: Id<UserMarker>, guild_id: Id<GuildMarker>) -> anyhow::Result<Option<User>> {
        let member = match sqlx::query_file!("queries/member_fetch.sql", guild_id.get() as i64, user_id.get() as i64)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(Some(member)) => member,
            Ok(None) => return Ok(None),
            Err(why) => return Err(why.into()),
        };

        let mut roles = HashMap::new();
        while let Ok(Some(role)) = sqlx::query_file!("queries/member_fetch_roles.sql", guild_id.get() as i64, user_id.get() as i64)
            .fetch(&self.pool)
            .try_next()
            .await
        {
            roles.insert(
                Id::new(role.role_id as u64),
                Role {
                    data: Some(RoleData { deleted: role.deleted }),
                    colour: role.colour as u32,
                    hoist: role.hoist,
                    icon: match role.icon {
                        Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                        None => None,
                    },
                    role_id: Id::new(role.role_id as u64),
                    guild_id: Id::new(role.guild_id as u64),
                    managed: role.managed,
                    mentionable: role.mentionable,
                    name: role.role_name,
                    permissions: Permissions::from_bits_retain(role.permissions as u64),
                    position: role.position,
                    flags: RoleFlags::from_bits_retain(role.role_flags as u64),
                    tags: role.tags.map(|x| x.0),
                    unicode_emoji: role.unicode_emoji,
                },
            );
        }

        let user = User {
            data: Some(UserData {
                user_id: Id::new(member.user_id as u64),
                permissions: member.user_permissions.into(),
                gender: member.gender.map(|x| x.into()),
                sexuality: member.sexuality.map(|x| x.into()),
            }),
            member: Some(Member {
                roles: roles.values().map(|x|x.role_id).collect(),
                data: Some(MemberData {
                    guild_everyone_role_permissions: Permissions::from_bits_retain(member.guild_everyone_role_permissions as u64),
                    guild_owner_id: Id::new(member.guild_owner_id as u64),
                    guild_owner: member.guild_owner,
                    left_at: member.left_at,
                    roles,
                    guild_id,
                    user_id,
                }),
                avatar: match member.member_avatar {
                    Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                boosting_since: match member.boosting_since {
                    Some(timestamp) => Some(Timestamp::from_secs(timestamp.unix_timestamp())?),
                    None => None,
                },
                communication_disabled_until: match member.communication_disabled_until {
                    Some(timestamp) => Some(Timestamp::from_secs(timestamp.unix_timestamp())?),
                    None => None,
                },
                joined_at: Timestamp::from_secs(member.joined_at.unix_timestamp())?,
                deafened: member.deafened,
                flags: MemberFlags::from_bits_retain(member.member_flags as u64),
                guild_id,
                muted: member.muted,
                nickname: member.nickname,
                pending: member.pending,
                user_id,
            }),
            accent_colour: member.accent_colour.map(|x| x as u32),
            avatar_decoration: match member.avatar_decoration {
                Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                None => None,
            },
            avatar: match member.user_avatar {
                Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                None => None,
            },
            banner: match member.user_banner {
                Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                None => None,
            },
            bot: member.bot,
            discriminator: member.discriminator as u16,
            email: member.email,
            flags: member.user_flags.map(|x| UserFlags::from_bits_retain(x as u64)),
            global_name: member.global_name,
            locale: member.locale,
            mfa_enabled: member.mfa_enabled,
            name: member.user_name,
            premium_type: member.premium_type.map(|x| PremiumType::from(x as u8)),
            public_flags: member.public_flags.map(|x| UserFlags::from_bits_retain(x as u64)),
            system: member.user_system,
            user_id: Id::new(member.user_id as u64),
            verified: member.verified,
        };

        Ok(Some(user))
    }
}
