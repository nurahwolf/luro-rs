use std::collections::HashMap;

use anyhow::Context;
use futures_util::TryStreamExt;
use sqlx::types::Json;
use tracing::warn;
use twilight_model::guild::RoleTags;
use twilight_model::id::marker::{GuildMarker, UserMarker};
use twilight_model::{
    guild::{Permissions, RoleFlags},
    id::Id,
};

use crate::{LuroDatabase, LuroMember, LuroRole, LuroUser, LuroUserData, LuroUserPermissions, LuroUserType};

impl LuroDatabase {
    pub async fn get_member(&self, user_id: Id<UserMarker>, guild_id: Id<GuildMarker>) -> anyhow::Result<LuroUser> {
        let mut luro_user: Option<LuroUser> = None;
        let mut result = sqlx::query_file!("queries/luro_user/get_luro_member.sql", guild_id.get() as i64, user_id.get() as i64).fetch(&self.pool);

        while let Ok(record) = result.try_next().await {
            // If we got no records, try parsing with no roles
            let db_user = match record {
                Some(user) => user,
                None => return get_member_no_roles(self, user_id, guild_id).await
            };

            match luro_user {
                Some(ref mut luro_user) => {
                    if let Some(ref mut member) = luro_user.member {
                        member.roles.insert(
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
                        );
                    }
                }
                None => {
                    luro_user = Some(LuroUser {
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
                    })
                }
            }
        }

        luro_user.context("Expected a luro_user to have been constructed")
    }
}

/// Attempt to get a member again, but this time skip the check for roles. Due to the way the records are structured, this can return a hit if the user is there, but the user has no roles in the DB.
async fn get_member_no_roles(db: &LuroDatabase, user_id: Id<UserMarker>, guild_id: Id<GuildMarker>) -> anyhow::Result<LuroUser> {
    let result = sqlx::query_file!("queries/luro_user/get_luro_member_no_roles.sql", guild_id.get() as i64, user_id.get() as i64).fetch_optional(&db.pool).await;

    if let Ok(Some(db_user)) = result {
        return Ok(LuroUser {
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
                roles: HashMap::new(),
                user_id: db_user.user_id,
            }),
            instance: LuroUserType::DbMemberNoRoles,
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
        });
    }

    warn!("fetch_member - Failed to fetch member `{user_id}` of guild `{guild_id}`, falling back to Twilight");
    match db.twilight_client.guild_member(guild_id, user_id).await {
        Ok(member) => Ok((member.model().await?, guild_id).into()),
        Err(why) => {
            warn!(why = ?why, "fetch_member - Failed to fetch member `{user_id}` of guild `{guild_id}` using the API, are they not a member of that guild?");
            db.get_user(user_id).await
        }
    }
}
