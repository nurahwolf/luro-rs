use sqlx::Error;
use twilight_model::id::{marker::UserMarker, Id};

use crate::{DatabaseUser, LuroDatabase, LuroUserPermissions};

impl LuroDatabase {
    pub async fn get_user(&self, user_id: &Id<UserMarker>) -> Result<Option<DatabaseUser>, Error> {
        sqlx::query_as!(
            DatabaseUser,
            "SELECT
                accent_colour,
                user_avatar,
                avatar_decoration,
                banner,
                bot,
                characters,
                discriminator,
                email,
                user_flags,
                global_name,
                locale,
                message_edits,
                messages,
                mfa_enabled,
                name,
                premium_type,
                public_flags,
                system,
                user_id,
                user_permissions as \"user_permissions: LuroUserPermissions\",
                verified,
                warnings,
                words_average,
                words_count
            FROM
                users
            WHERE
                user_id = $1",
            user_id.get() as i64
        )
        .fetch_optional(&self.pool)
        .await
    }
}
