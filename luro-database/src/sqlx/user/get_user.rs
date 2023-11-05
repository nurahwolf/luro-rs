use tracing::{error, warn};
use twilight_model::{
    id::{marker::UserMarker, Id},
    user::{PremiumType, UserFlags},
    util::ImageHash,
};

use crate::{LuroDatabase, LuroUser, LuroUserData, LuroUserPermissions, LuroUserType, Gender, Sexuality};

impl LuroDatabase {
    pub async fn get_user(&self, user_id: Id<UserMarker>) -> anyhow::Result<LuroUser> {
        let query = sqlx::query_file!("queries/luro_user/get_luro_user.sql", user_id.get() as i64)
            .fetch_optional(&self.pool)
            .await;

        if let Ok(Some(user)) = query {
            return Ok(LuroUser {
                data: Some(LuroUserData {
                    permissions: user.user_permissions,
                    gender: user.gender,
                    sexuality: user.sexuality,
                }),
                member: None,
                instance: LuroUserType::DbUser,
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
            });
        }

        warn!("fetch_user - Failed to fetch user `{user_id}`, falling back to Twilight");
        let twilight_user = self.twilight_client.user(user_id).await?.model().await?;

        match self.update_user(twilight_user.clone()).await {
            Ok(_) => Ok(twilight_user.into()),
            Err(why) => {
                error!(why = ?why, "failed to sync user `{user_id}` to the database");
                Ok(twilight_user.into())
            }
        }
    }
}
