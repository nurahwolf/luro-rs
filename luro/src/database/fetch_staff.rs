use crate::models::User;

impl super::Database {
    pub async fn fetch_staff(&self) -> anyhow::Result<Vec<User>> {
        #[cfg(feature = "database-sqlx")]
        match fetch_staff_sqlx(self).await {
            Ok(data) => match data.is_empty() {
                true => tracing::warn!(
                    "No staff were returned from the database, falling back to hardcoded."
                ),
                false => return Ok(data),
            },
            Err(why) => tracing::error!(?why, "Error raised while trying to find staff"),
        };

        let mut staff = vec![];
        for staff_id in crate::BOT_OWNERS {
            staff.push(self.fetch_user(staff_id).await?);
        }

        Ok(staff)
    }
}

#[cfg(feature = "database-sqlx")]
async fn fetch_staff_sqlx(db: &super::Database) -> anyhow::Result<Vec<User>> {
    use futures_util::StreamExt;

    use crate::models::{Gender, Sexuality, UserContext, UserPermissions};

    // Query the database and attempt to fetch the user, returning if an error is raised.
    let mut users = vec![];
    let mut query =
        sqlx::query_file!("src/database/sqlx_queries/user/user_fetch_staff.sql").fetch(&db.pool);

    while let Some(query) = query.next().await {
        let user = match query {
            Ok(user) => user,
            Err(why) => {
                tracing::warn!(?why, "Failed to fetch staff member from DB");
                continue;
            }
        };

        users.push(User::User(UserContext {
            accent_colour: user.accent_colour.map(|x| x as u32),
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
            flags: user
                .user_flags
                .map(|x| twilight_model::user::UserFlags::from_bits_retain(x as u64)),
            global_name: user.global_name,
            locale: user.locale,
            mfa_enabled: user.mfa_enabled,
            name: user.user_name,
            premium_type: user
                .premium_type
                .map(|x| twilight_model::user::PremiumType::from(x as u8)),
            public_flags: user
                .public_flags
                .map(|x| twilight_model::user::UserFlags::from_bits_retain(x as u64)),
            system: user.user_system,
            user_id: twilight_model::id::Id::new(user.user_id as u64),
            verified: user.verified,
            gender: user.gender,
            permissions: user.user_permissions,
            sexuality: user.sexuality,
        }))
    }

    Ok(users)
}
