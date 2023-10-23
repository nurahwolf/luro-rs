use sqlx::Error;
use twilight_model::id::{marker::UserMarker, Id};

use crate::{LuroDatabase, LuroUser, LuroUserData, LuroUserPermissions, LuroUserType};

impl LuroDatabase {
    pub async fn get_user(&self, user_id: &Id<UserMarker>) -> Result<Option<LuroUser>, Error> {
        let db_user = sqlx::query_file!("queries/luro_user/get_luro_user.sql", user_id.get() as i64)
            .fetch_optional(&self.pool)
            .await?;
        // Check to see if we have an item in the iterator, if not return a None type.
        // If we do have some data, setup a Luro user

        match db_user {
            Some(db_user) => Ok(Some(LuroUser {
                data: Some(LuroUserData {
                    permissions: db_user.user_permissions,
                }),
                member: None,
                instance: LuroUserType::DbUser,
                accent_colour: db_user.accent_colour,
                avatar_decoration: db_user.avatar_decoration,
                avatar: db_user.user_avatar,
                banner: db_user.user_banner,
                bot: db_user.bot,
                discriminator: db_user.discriminator,
                email: db_user.email,
                flags: db_user.user_flags,
                global_name: db_user.global_name,
                locale: db_user.locale,
                mfa_enabled: db_user.mfa_enabled,
                name: db_user.user_name,
                premium_type: db_user.premium_type,
                public_flags: db_user.public_flags,
                system: db_user.user_system,
                user_id: db_user.user_id,
                verified: db_user.verified,
            })),
            None => Ok(None),
        }
    }
}
