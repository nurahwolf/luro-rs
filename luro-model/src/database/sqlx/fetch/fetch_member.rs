use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

use crate::{
    database::sqlx::{Database, Error},
    gender::{Gender, Sexuality},
    user::{MemberContext, UserContext, UserPermissions},
};

impl Database {
    // Fetch a member from the database. Note that due to the need to query the database twice, this does not get roles automatically.
    pub async fn fetch_member(&self, guild_id: Id<GuildMarker>, user_id: Id<UserMarker>) -> Result<Option<MemberContext>, Error> {
        let g_id = guild_id.get() as i64;
        let u_id = user_id.get() as i64;

        // Fetch the member from the database
        let member = sqlx::query_file!("queries/member/member_fetch.sql", g_id, u_id).fetch_optional(&self.pool);
        let member = match member.await {
            Ok(Some(member)) => member,
            Ok(None) => return Ok(None),
            Err(why) => return Err(why.into()),
        };

        // Create a user context
        let user = UserContext {
            twilight_user: twilight_model::user::User {
                accent_color: member.accent_colour.map(|x| x as u32),
                avatar_decoration: match member.avatar_decoration {
                    Some(img) => Some(twilight_model::util::ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                avatar: match member.user_avatar {
                    Some(img) => Some(twilight_model::util::ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                banner: match member.user_banner {
                    Some(img) => Some(twilight_model::util::ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                bot: member.bot,
                discriminator: member.discriminator as u16,
                email: member.email,
                flags: member
                    .user_flags
                    .map(|x| twilight_model::user::UserFlags::from_bits_retain(x as u64)),
                global_name: member.global_name,
                locale: member.locale,
                mfa_enabled: member.mfa_enabled,
                name: member.user_name,
                premium_type: member.premium_type.map(|x| twilight_model::user::PremiumType::from(x as u8)),
                public_flags: member
                    .public_flags
                    .map(|x| twilight_model::user::UserFlags::from_bits_retain(x as u64)),
                system: member.user_system,
                id: user_id,
                verified: member.verified,
            },
            gender: member.gender,
            user_type: member.user_permissions,
            sexuality: member.sexuality,
        };

        // Create a member context, based on the previous user context
        let member = MemberContext {
            user: user.clone(),
            twilight_member: twilight_model::guild::Member {
                avatar: match member.member_avatar {
                    Some(img) => Some(twilight_model::util::ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                communication_disabled_until: match member.communication_disabled_until {
                    Some(timestamp) => Some(twilight_model::util::Timestamp::from_secs(timestamp.unix_timestamp())?),
                    None => None,
                },
                deaf: member.deafened,
                flags: twilight_model::guild::MemberFlags::from_bits_retain(member.member_flags as u64),
                // TODO: Change to optional
                joined_at: Some(twilight_model::util::Timestamp::from_secs(member.joined_at.unix_timestamp())?),
                mute: member.muted,
                nick: member.nickname,
                pending: member.pending,
                premium_since: match member.boosting_since {
                    Some(timestamp) => Some(twilight_model::util::Timestamp::from_secs(timestamp.unix_timestamp())?),
                    None => None,
                },
                roles: vec![],
                user: user.twilight_user,
            },
            guild_id,
            roles: vec![],
            everyone_role: None,
        };

        Ok(Some(member))
    }
}
