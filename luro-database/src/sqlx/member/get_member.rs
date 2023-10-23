use std::collections::HashMap;

use sqlx::types::Json;
use sqlx::Error;
use twilight_model::guild::RoleTags;
use twilight_model::{
    guild::{Permissions, RoleFlags},
    id::Id,
};

use crate::{LuroDatabase, LuroMember, LuroRole, LuroUser, LuroUserData, LuroUserPermissions, LuroUserType};

impl LuroDatabase {
    pub async fn get_member(&self, user_id: i64, guild_id: i64) -> Result<Option<LuroUser>, Error> {
        // Fetch our results into an iterator
        let mut result = sqlx::query_file!("queries/luro_user/get_luro_member.sql", guild_id, user_id)
            .fetch_all(&self.pool)
            .await?
            .into_iter();

        // Check to see if we have an item in the iterator, if not return a None type.
        // If we do have some data, setup a Luro user

        let mut luro_user = match result.next() {
            Some(db_user) => LuroUser {
                data: Some(LuroUserData {
                    permissions: db_user.user_permissions,
                }),
                member: Some(LuroMember {
                    left_at: db_user.left_at,
                    avatar: db_user.member_avatar,
                    boosting_since: db_user.boosting_since,
                    communication_disabled_until: db_user.communication_disabled_until,
                    joined_at: db_user.joined_at,
                    deafened: db_user.deafened,
                    flags: db_user.member_flags,
                    guild_id: db_user.guild_id,
                    muted: db_user.muted,
                    nickname: db_user.nickname,
                    pending: db_user.pending,
                    roles: HashMap::from([(
                        Id::new(db_user.role_id as u64),
                        LuroRole {
                            deleted: db_user.deleted,
                            guild_id: Id::new(db_user.guild_id as u64),
                            colour: db_user.colour as u32,
                            hoist: db_user.hoist,
                            icon: db_user.icon,
                            id: Id::new(db_user.role_id as u64),
                            managed: db_user.managed,
                            mentionable: db_user.mentionable,
                            name: db_user.role_name,
                            permissions: Permissions::from_bits_retain(db_user.permissions as u64),
                            position: db_user.position,
                            flags: RoleFlags::from_bits_retain(db_user.role_flags as u64),
                            tags: db_user.tags.map(|x| x.0),
                            unicode_emoji: db_user.unicode_emoji,
                        },
                    )]),
                    user_id: db_user.user_id,
                }),
                instance: LuroUserType::DbMember,
                accent_colour: db_user.accent_colour,
                avatar_decoration: db_user.avatar_decoration,
                avatar: db_user.user_avatar,
                banner: db_user.user_banner,
                bot: db_user.bot,
                discriminator: db_user.discriminator,
                email: db_user.email,
                flags: db_user.user_flags,
                global_name: db_user.global_name,
                locale: db_user.locale,
                mfa_enabled: db_user.mfa_enabled,
                name: db_user.user_name,
                premium_type: db_user.premium_type,
                public_flags: db_user.public_flags,
                system: db_user.user_system,
                user_id: db_user.user_id,
                verified: db_user.verified,
            },
            None => return Ok(None),
        };

        // Drain the rest of the iterator and finish crafting our roles
        for role in result {
            if let Some(ref mut member) = luro_user.member {
                member.roles.insert(
                    Id::new(role.role_id as u64),
                    LuroRole {
                        deleted: role.deleted,
                        guild_id: Id::new(role.guild_id as u64),
                        colour: role.colour as u32,
                        hoist: role.hoist,
                        icon: role.icon,
                        id: Id::new(role.role_id as u64),
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
        }

        Ok(Some(luro_user))
    }
}
