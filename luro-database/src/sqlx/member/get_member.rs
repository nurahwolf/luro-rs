use std::collections::HashMap;

use futures_util::TryStreamExt;
use sqlx::types::Json;
use tracing::{error, warn};
use twilight_model::guild::{RoleTags, MemberFlags};
use twilight_model::id::marker::{GuildMarker, UserMarker};

use twilight_model::util::{ImageHash, Timestamp};
use twilight_model::{
    guild::{Permissions, RoleFlags},
    id::Id,
};

use crate::luro::luro_role_data::LuroRoleData;
use crate::{LuroDatabase, LuroMember, LuroRole, LuroUser, LuroUserPermissions, LuroUserType, LuroMemberData};

impl LuroDatabase {
    pub async fn get_member(&self, user_id: Id<UserMarker>, guild_id: Id<GuildMarker>) -> anyhow::Result<LuroUser> {
        let mut luro_user = self.get_user(user_id).await?;
        let mut result =
            sqlx::query_file!("queries/luro_user/get_luro_member.sql", guild_id.get() as i64, user_id.get() as i64).fetch(&self.pool);

        while let Ok(Some(db_user)) = result.try_next().await {
            match luro_user.member {
                Some(ref mut member) => {
                    member.roles.push(Id::new(db_user.role_id as u64));
                    if let Some(ref mut data) = member.data {
                        data.roles.insert(
                            Id::new(db_user.role_id as u64),
                            LuroRole {
                                data: Some(LuroRoleData { deleted: db_user.deleted }),
                                colour: db_user.colour as u32,
                                hoist: db_user.hoist,
                                icon: match db_user.icon {
                                    Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                                    None => None,
                                },
                                role_id: Id::new(db_user.role_id as u64),
                                guild_id: Id::new(db_user.guild_id as u64),
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
                    luro_user.instance = LuroUserType::DbMember;
                    luro_user.member = Some(LuroMember {
                        data: Some(LuroMemberData {
                            guild_owner: db_user.guild_owner,
                            left_at: db_user.left_at,
                            roles: HashMap::from([(
                                Id::new(db_user.role_id as u64),
                                LuroRole {
                                    data: Some(LuroRoleData { deleted: db_user.deleted }),
                                    colour: db_user.colour as u32,
                                    hoist: db_user.hoist,
                                    icon: match db_user.icon {
                                        Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                                        None => None,
                                    },
                                    role_id: Id::new(db_user.role_id as u64),
                                    guild_id: Id::new(db_user.guild_id as u64),
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
                            guild_id,
                            user_id,
                        }),
                        avatar: match db_user.member_avatar {
                            Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                            None => None,
                        },
                        boosting_since: match db_user.boosting_since {
                            Some(timestamp) => Some(Timestamp::from_secs(timestamp.unix_timestamp())?),
                            None => None,
                        },
                        communication_disabled_until: match db_user.communication_disabled_until {
                            Some(timestamp) => Some(Timestamp::from_secs(timestamp.unix_timestamp())?),
                            None => None,
                        },
                        joined_at: Timestamp::from_secs(db_user.joined_at.unix_timestamp())?,
                        deafened: db_user.deafened,
                        flags: MemberFlags::from_bits_retain(db_user.member_flags as u64),
                        guild_id,
                        muted: db_user.muted,
                        nickname: db_user.nickname,
                        pending: db_user.pending,
                        user_id,
                        roles: vec![Id::new(db_user.role_id as u64)],
                    });
                }
            };
        }

        if luro_user.member.is_none() {
            // If we got no records, try parsing with no roles
            return get_member_no_roles(self, user_id, guild_id).await;
        }

        Ok(luro_user)
    }
}

/// Attempt to get a member again, but this time skip the check for roles. Due to the way the records are structured, this can return a hit if the user is there, but the user has no roles in the DB.
async fn get_member_no_roles(db: &LuroDatabase, user_id: Id<UserMarker>, guild_id: Id<GuildMarker>) -> anyhow::Result<LuroUser> {
    let mut luro_user = db.get_user(user_id).await?;

    let result = sqlx::query_file!(
        "queries/luro_user/get_luro_member_no_roles.sql",
        guild_id.get() as i64,
        user_id.get() as i64
    )
    .fetch_optional(&db.pool)
    .await;

    if let Ok(Some(db_user)) = result {
        luro_user.instance = LuroUserType::DbMemberNoRoles;
        luro_user.member = Some(LuroMember {
            data: Some(LuroMemberData {
                left_at: db_user.left_at,
                roles: HashMap::new(),
                guild_id,
                user_id,
                guild_owner: db_user.guild_owner,
            }),
            avatar: match db_user.member_avatar {
                Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                None => None,
            },
            boosting_since: match db_user.boosting_since {
                Some(timestamp) => Some(Timestamp::from_secs(timestamp.unix_timestamp())?),
                None => None,
            },
            communication_disabled_until: match db_user.communication_disabled_until {
                Some(timestamp) => Some(Timestamp::from_secs(timestamp.unix_timestamp())?),
                None => None,
            },
            joined_at: Timestamp::from_secs(db_user.joined_at.unix_timestamp())?,
            deafened: db_user.deafened,
            flags: MemberFlags::from_bits_retain(db_user.member_flags as u64),
            guild_id,
            muted: db_user.muted,
            nickname: db_user.nickname,
            pending: db_user.pending,
            roles: vec![],
            user_id,
        });
        return Ok(luro_user);
    }

    warn!("fetch_member - Failed to fetch member `{user_id}` of guild `{guild_id}`, falling back to Twilight");
    let member = match db.twilight_client.guild_member(guild_id, user_id).await {
        Ok(member) => member.model().await?,
        Err(why) => {
            warn!(why = ?why, "fetch_member - Failed to fetch member `{user_id}` of guild `{guild_id}` using the API, are they not a member of that guild?");
            return db.get_user(user_id).await;
        }
    };

    for role_id in member.roles.clone() {
        if let Err(why) = db.update_guild_member_roles(guild_id, role_id, user_id).await {
            error!(why = ?why, "fetch_member - failed to sync role `{role_id}` of member `{user_id}` of guild `{guild_id}` to the database!");
        }
    }

    if let Err(why) = db.update_member((guild_id, member.clone())).await {
        error!(why = ?why, "fetch_member - failed to sync member `{user_id}` of guild `{guild_id}` to the database!");
    }

    Ok((member, guild_id).into())
}
