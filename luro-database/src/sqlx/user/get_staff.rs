use crate::{DatabaseUser, LuroDatabase, LuroUserPermissions};

impl LuroDatabase {
    pub async fn get_staff(&self) -> Result<Vec<DatabaseUser>, sqlx::Error> {
        sqlx::query_as!(
            DatabaseUser,
            "SELECT
                accent_colour,
                user_avatar,
                avatar_decoration,
                user_banner,
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
                user_name,
                premium_type,
                public_flags,
                user_system,
                user_id,
                user_permissions as \"user_permissions: LuroUserPermissions\",
                verified,
                warnings,
                words_average,
                words_count
            FROM
                users
            WHERE
                user_permissions = 'OWNER' 
                    or
                user_permissions = 'ADMINISTRATOR'",
        )
        .fetch_all(&self.pool)
        .await
    }
}
