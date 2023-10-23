
use sqlx::Error;




use crate::LuroUserData;
use crate::{DatabaseUser, LuroDatabase, LuroUserPermissions};

impl LuroDatabase {
    pub async fn update_user_data(&self, user_id: i64, data: LuroUserData) -> Result<DatabaseUser, Error> {
        sqlx::query_as!(
            DatabaseUser,
            "
            INSERT INTO users (user_id, user_permissions)
            VALUES ($1, $2)
            ON CONFLICT (user_id)
            DO UPDATE SET
                user_permissions = $2
            RETURNING
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
            ",
            user_id,
            data.permissions as _,
        )
        .fetch_one(&self.pool)
        .await
    }
}
