use crate::{DatabaseUser, LuroDatabase, LuroUserPermissions};
use sqlx::types::Json;
use twilight_model::user::PremiumType;
use twilight_model::user::UserFlags;
use twilight_model::util::ImageHash;

impl LuroDatabase {
    pub async fn get_staff(&self) -> Result<Vec<DatabaseUser>, sqlx::Error> {
        sqlx::query_as!(
            DatabaseUser,
            "SELECT
                accent_colour,
                avatar as \"avatar: Json<ImageHash>\",
                avatar_decoration as \"avatar_decoration: Json<ImageHash>\",
                banner as \"banner: Json<ImageHash>\",
                bot,
                characters,
                discriminator,
                email,
                flags as \"flags: Json<UserFlags>\",
                global_name,
                locale,
                message_edits,
                messages,
                mfa_enabled,
                name,
                premium_type as \"premium_type: Json<PremiumType>\",
                public_flags as \"public_flags: Json<UserFlags>\",
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
                user_permissions = 'OWNER' 
                    or
                user_permissions = 'ADMINISTRATOR'",
        )
        .fetch_all(&self.pool)
        .await
    }
}
