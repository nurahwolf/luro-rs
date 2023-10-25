use tracing::{warn, error};
use twilight_model::id::{marker::UserMarker, Id};

use crate::{LuroDatabase, LuroUser, LuroUserData, LuroUserPermissions, LuroUserType};

impl LuroDatabase {
    pub async fn get_user(&self, user_id: Id<UserMarker>) -> anyhow::Result<LuroUser> {
        let query = sqlx::query_file!("queries/luro_user/get_luro_user.sql", user_id.get() as i64)
            .fetch_optional(&self.pool)
            .await;

        if let Ok(Some(user)) = query {
            return  Ok(LuroUser {
                data: Some(LuroUserData {
                    permissions: user.user_permissions,
                }),
                member: None,
                instance: LuroUserType::DbUser,
                accent_colour: user.accent_colour,
                avatar_decoration: user.avatar_decoration,
                avatar: user.user_avatar,
                banner: user.user_banner,
                bot: user.bot,
                discriminator: user.discriminator,
                email: user.email,
                flags: user.user_flags,
                global_name: user.global_name,
                locale: user.locale,
                mfa_enabled: user.mfa_enabled,
                name: user.user_name,
                premium_type: user.premium_type,
                public_flags: user.public_flags,
                system: user.user_system,
                user_id: user.user_id,
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
            },
        }
    }
}
