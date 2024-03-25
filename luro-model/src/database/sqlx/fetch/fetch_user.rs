use twilight_model::id::{marker::UserMarker, Id};

use crate::{
    database::sqlx::{Database, Error},
    gender::{Gender, Sexuality},
    user::{UserContext, UserPermissions},
};

impl Database {
    pub async fn fetch_user(&self, user_id: Id<UserMarker>) -> Result<Option<UserContext>, Error> {
        // Query the database and attempt to fetch the user, returning if an error is raised.
        let user = sqlx::query_file!("queries/luro_user/get_luro_user.sql", user_id.get() as i64)
            .fetch_optional(&self.pool)
            .await?;

        // Return early if no result was returned.
        let user = match user {
            Some(user) => user,
            None => return Ok(None),
        };

        let user = UserContext {
            twilight_user: twilight_model::user::User {
                accent_color: user.accent_colour.map(|x| x as u32),
                avatar_decoration: match user.avatar_decoration {
                    Some(img) => Some(twilight_model::util::ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                avatar: match user.user_avatar {
                    Some(img) => Some(twilight_model::util::ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                banner: match user.user_banner {
                    Some(img) => Some(twilight_model::util::ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                bot: user.bot,
                discriminator: user.discriminator as u16,
                email: user.email,
                flags: user.user_flags.map(|x| twilight_model::user::UserFlags::from_bits_retain(x as u64)),
                global_name: user.global_name,
                locale: user.locale,
                mfa_enabled: user.mfa_enabled,
                name: user.user_name,
                premium_type: user.premium_type.map(|x| twilight_model::user::PremiumType::from(x as u8)),
                public_flags: user
                    .public_flags
                    .map(|x| twilight_model::user::UserFlags::from_bits_retain(x as u64)),
                system: user.user_system,
                id: twilight_model::id::Id::new(user.user_id as u64),
                verified: user.verified,
            },
            gender: user.gender,
            user_type: user.user_permissions,
            sexuality: user.sexuality,
        };

        Ok(Some(user))
    }
}
