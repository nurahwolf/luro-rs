use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::models::{interaction::InteractionResult, MemberContext};

use super::Database;

impl Database {
    pub async fn fetch_member(
        &self,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> InteractionResult<MemberContext> {
        // #[cfg(feature = "database-sqlx")]
        // match fetch_member_sqlx(self, guild_id, user_id).await {
        //     Ok(Some(user)) => return Ok(user),
        //     Ok(None) => tracing::debug!(
        //         "[User `{user_id}`, Guild `{guild_id}`] was not found in the database."
        //     ),
        //     Err(why) => tracing::error!(
        //         ?why,
        //         "Error raised while trying to find [User `{user_id}`, Guild `{guild_id}`]"
        //     ),
        // };

        let mut user_roles = vec![];
        let mut everyone_role = None;
        let member = self
            .twilight_client
            .guild_member(guild_id, user_id)
            .await?
            .model()
            .await?;
        let guild = self.twilight_client.guild(guild_id).await?.model().await?;
        let guild_roles = self.twilight_client.roles(guild_id).await?.model().await?;
        for role in guild_roles {
            if role.id.get() == guild_id.get() {
                everyone_role = Some(role);
                continue;
            }

            if member.roles.contains(&role.id) {
                user_roles.push((guild_id, role).into())
            }
        }

        let everyone_role = match everyone_role {
            Some(everyone_role) => (guild_id, everyone_role).into(),
            None => todo!(),
        };

        user_roles.sort();

        Ok(MemberContext::twilight(
            member,
            guild_id,
            user_roles,
            everyone_role,
            guild.owner_id,
        ))
    }
}

// #[cfg(feature = "database-sqlx")]
// pub async fn fetch_member_sqlx(
//     db: &Database,
//     guild_id: Id<GuildMarker>,
//     user_id: Id<UserMarker>,
// ) -> Result<Option<MemberContext>, InteractionError> {
//     use crate::models::{Gender, Role, Sexuality, UserContext, UserPermissions};
//     use futures_util::TryStreamExt;
//     use sqlx::types::Json;
//     use twilight_model::{
//         guild::RoleTags,
//         user::{PremiumType, UserFlags},
//     };

//     let mut roles_query = sqlx::query_file!(
//         "src/database/sqlx_queries/member/member_fetch_roles.sql",
//         guild_id.get() as i64,
//         user_id.get() as i64
//     )
//     .fetch(&db.pool);

//     let member = match sqlx::query_file!(
//         "src/database/sqlx_queries/member/member_fetch.sql",
//         guild_id.get() as i64,
//         user_id.get() as i64
//     )
//     .fetch_optional(&db.pool)
//     .await
//     {
//         Ok(Some(member)) => member,
//         Ok(None) => return Ok(None),
//         Err(why) => return Err(why.into()),
//     };

//     let mut roles = std::collections::HashMap::new();
//     let mut everyone_role = None;
//     while let Ok(Some(role)) = roles_query.try_next().await {
//         tracing::debug!("Role: {role:#?}");

//         if role.role_id as u64 == guild_id.get() {
//             everyone_role = Some(Role {
//                 deleted: Some(role.deleted),
//                 role: twilight_model::guild::Role {
//                     color: role.colour as u32,
//                     hoist: role.hoist,
//                     icon: match role.icon {
//                         Some(img) => match twilight_model::util::ImageHash::parse(img.as_bytes()) {
//                             Ok(hash) => Some(hash),
//                             Err(why) => {
//                                 tracing::warn!("role_icon - Failed to parse image: {why}");
//                                 None
//                             }
//                         },
//                         None => None,
//                     },
//                     id: twilight_model::id::Id::new(role.role_id as u64),
//                     managed: role.managed,
//                     mentionable: role.mentionable,
//                     name: role.role_name,
//                     permissions: twilight_model::guild::Permissions::from_bits_retain(
//                         role.permissions as u64,
//                     ),
//                     position: role.position,
//                     flags: twilight_model::guild::RoleFlags::from_bits_retain(
//                         role.role_flags as u64,
//                     ),
//                     tags: role.tags.map(|x| x.0),
//                     unicode_emoji: role.unicode_emoji,
//                 },
//                 guild_id,
//             });
//             continue;
//         }

//         roles.insert(
//             twilight_model::id::Id::new(role.role_id as u64),
//             Role {
//                 deleted: Some(role.deleted),
//                 role: twilight_model::guild::Role {
//                     color: role.colour as u32,
//                     hoist: role.hoist,
//                     icon: match role.icon {
//                         Some(img) => match twilight_model::util::ImageHash::parse(img.as_bytes()) {
//                             Ok(hash) => Some(hash),
//                             Err(why) => {
//                                 tracing::warn!("role_icon - Failed to parse image: {why}");
//                                 None
//                             }
//                         },
//                         None => None,
//                     },
//                     id: twilight_model::id::Id::new(role.role_id as u64),
//                     managed: role.managed,
//                     mentionable: role.mentionable,
//                     name: role.role_name,
//                     permissions: twilight_model::guild::Permissions::from_bits_retain(
//                         role.permissions as u64,
//                     ),
//                     position: role.position,
//                     flags: twilight_model::guild::RoleFlags::from_bits_retain(
//                         role.role_flags as u64,
//                     ),
//                     tags: role.tags.map(|x| x.0),
//                     unicode_emoji: role.unicode_emoji,
//                 },
//                 guild_id,
//             },
//         );
//     }

//     let public_flags = member.public_flags;
//     let member = MemberContext {
//         user: UserContext {
//             accent_colour: member.accent_colour.map(|x| x as u32),
//             avatar_decoration: match member.avatar_decoration {
//                 Some(img) => Some(twilight_model::util::ImageHash::parse(img.as_bytes())?),
//                 None => None,
//             },
//             avatar: match member.user_avatar {
//                 Some(img) => Some(twilight_model::util::ImageHash::parse(img.as_bytes())?),
//                 None => None,
//             },
//             banner: match member.user_banner {
//                 Some(img) => Some(twilight_model::util::ImageHash::parse(img.as_bytes())?),
//                 None => None,
//             },
//             bot: member.bot,
//             discriminator: member.discriminator as u16,
//             email: member.email,
//             flags: member
//                 .user_flags
//                 .map(|x| twilight_model::user::UserFlags::from_bits_retain(x as u64)),
//             global_name: member.global_name,
//             locale: member.locale,
//             mfa_enabled: member.mfa_enabled,
//             name: member.user_name,
//             premium_type: member.premium_type.map(|x| PremiumType::from(x as u8)),
//             public_flags: public_flags.map(|x| UserFlags::from_bits_retain(x as u64)),
//             system: member.user_system,
//             user_id: Id::new(member.user_id as u64),
//             verified: member.verified,
//             gender: member.gender,
//             permissions: member.user_permissions,
//             sexuality: member.sexuality,
//         },
//         avatar: match member.member_avatar {
//             Some(img) => Some(twilight_model::util::ImageHash::parse(img.as_bytes())?),
//             None => None,
//         },
//         banner: None,
//         communication_disabled_until: match member.communication_disabled_until {
//             Some(timestamp) => Some(twilight_model::util::Timestamp::from_secs(
//                 timestamp.unix_timestamp(),
//             )?),
//             None => None,
//         },
//         deafened: member.deafened,
//         flags: twilight_model::guild::MemberFlags::from_bits_retain(member.member_flags as u64),
//         // TODO: Change to optional
//         joined_at: Some(twilight_model::util::Timestamp::from_secs(
//             member.joined_at.unix_timestamp(),
//         )?),
//         muted: member.muted,
//         nickname: member.nickname,
//         pending: member.pending,
//         boosting_since: match member.boosting_since {
//             Some(timestamp) => Some(twilight_model::util::Timestamp::from_secs(
//                 timestamp.unix_timestamp(),
//             )?),
//             None => None,
//         },
//         role_id: roles.values().map(|x| x.role_id).collect(),
//         guild_id,
//         roles,
//     };

//     Ok(Some(member))
// }
