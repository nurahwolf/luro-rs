use twilight_model::id::{marker::UserMarker, Id};

use crate::models::{interaction::InteractionError, User};

use super::Database;

impl Database {
    pub async fn fetch_user(&self, id: Id<UserMarker>) -> Result<User, InteractionError> {
        #[cfg(feature = "database-sqlx")]
        match fetch_user_sqlx(self, id).await {
            Ok(Some(user)) => return Ok(user),
            Ok(None) => tracing::debug!("User `{id}` was not found in the database."),
            Err(why) => tracing::error!(?why, "Error raised while trying to find user `{id}`"),
        };

        Ok(self.twilight_client.user(id).await?.model().await?.into())
    }
}

#[cfg(feature = "database-sqlx")]
async fn fetch_user_sqlx(db: &Database, user_id: Id<UserMarker>) -> anyhow::Result<Option<User>> {
    use crate::models::{Gender, Sexuality, UserContext, UserPermissions};

    // Query the database and attempt to fetch the user, returning if an error is raised.
    let user = sqlx::query_file!(
        "src/database/sqlx_queries/luro_user/get_luro_user.sql",
        user_id.get() as i64
    )
    .fetch_optional(&db.pool)
    .await?;

    // Return early if no result was returned.
    let user = match user {
        Some(user) => user,
        None => return Ok(None),
    };

    // Craft the response into the expected output
    Ok(Some(User::User(UserContext {
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
    })))
}
