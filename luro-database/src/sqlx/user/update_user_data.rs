use sqlx::types::Json;
use sqlx::Error;
use twilight_model::user::PremiumType;
use twilight_model::user::UserFlags;
use twilight_model::util::ImageHash;

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
            ",
            user_id,
            data.permissions as _,
        )
        .fetch_one(&self.pool)
        .await
    }
}
