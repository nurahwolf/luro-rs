use futures_util::StreamExt;

use crate::{
    database::sqlx::{Database, Error},
    gender::{Gender, Sexuality},
    user::{User, UserContext, UserPermissions},
};

impl Database {
    pub async fn fetch_staff(&self) -> Result<Vec<User>, Error> {
        match fetch_staff(self).await {
            Ok(data) => match data.is_empty() {
                true => tracing::warn!("No staff were returned from the database, falling back to hardcoded."),
                false => return Ok(data),
            },
            Err(why) => tracing::error!(?why, "Error raised while trying to find staff"),
        };

        Ok(self.twilight_driver.fetch_staff().await?)
    }
}

async fn fetch_staff(db: &Database) -> Result<Vec<User>, Error> {
    let mut users = vec![];
    let mut query = sqlx::query_file!("src/database/sqlx/queries/user/user_fetch_staff.sql").fetch(&db.pool);

    while let Some(query) = query.next().await {
        let user = match query {
            Ok(user) => user,
            Err(why) => {
                tracing::warn!(?why, "Failed to fetch staff member from DB");
                continue;
            }
        };

        users.push(User::User(UserContext {
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
        }))
    }

    Ok(users)
}
