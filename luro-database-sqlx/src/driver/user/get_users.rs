use futures_util::TryStreamExt;
use luro_model::types::{User, UserData};
use twilight_model::{
    id::Id,
    user::{PremiumType, UserFlags},
    util::ImageHash,
};

use crate::types::{DbUserPermissions, DbGender, DbSexuality};

impl crate::SQLxDriver {
    pub async fn get_users(&self) -> anyhow::Result<Vec<User>> {
        let mut query = sqlx::query_file!("queries/users_fetch.sql").fetch(&self.pool);
        let mut users = vec![];

        while let Ok(Some(user)) = query.try_next().await {
            users.push(User {
                data: Some(UserData {
                    user_id: Id::new(user.user_id as u64),
                    permissions: user.user_permissions.into(),
                    gender: user.gender.map(|x|x.into()),
                    sexuality: user.sexuality.map(|x|x.into()),
                }),
                member: None,
                accent_colour: user.accent_colour.map(|x| x as u32),
                avatar_decoration: match user.avatar_decoration {
                    Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                avatar: match user.user_avatar {
                    Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                banner: match user.user_banner {
                    Some(img) => Some(ImageHash::parse(img.as_bytes())?),
                    None => None,
                },
                bot: user.bot,
                discriminator: user.discriminator as u16,
                email: user.email,
                flags: user.user_flags.map(|x| UserFlags::from_bits_retain(x as u64)),
                global_name: user.global_name,
                locale: user.locale,
                mfa_enabled: user.mfa_enabled,
                name: user.user_name,
                premium_type: user.premium_type.map(|x| PremiumType::from(x as u8)),
                public_flags: user.public_flags.map(|x| UserFlags::from_bits_retain(x as u64)),
                system: user.user_system,
                user_id: Id::new(user.user_id as u64),
                verified: user.verified,
            })
        }

        Ok(users)
    }
}
