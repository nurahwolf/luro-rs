use sqlx::types::Json;
use sqlx::Error;
use twilight_model::user::PremiumType;
use twilight_model::user::UserFlags;
use twilight_model::util::ImageHash;

use crate::LuroUser;
use crate::{DatabaseUser, LuroDatabase, LuroUserPermissions};

impl LuroDatabase {
    pub async fn handle_luro_user(&self, user: LuroUser) -> Result<DatabaseUser, Error> {
        sqlx::query_as!(
            DatabaseUser,
            "INSERT INTO users (
                accent_colour,
                avatar,
                banner,
                bot,
                discriminator,
                email,
                flags,
                locale,
                mfa_enabled,
                name,
                premium_type,
                public_flags,
                user_id,
                verified
            ) VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT
                (user_id)
            DO UPDATE SET
                accent_colour = $1,
                avatar = $2,
                banner = $3,
                bot = $4,
                discriminator = $5,
                email = $6,
                flags = $7,
                locale = $8,
                mfa_enabled = $9,
                name = $10,
                premium_type = $11,
                public_flags = $12,
                verified = $14
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
            user.accent_colour,
            user.avatar as _,
            user.banner as _,
            user.bot,
            user.discriminator,
            user.email,
            user.flags as _,
            user.locale,
            user.mfa_enabled,
            user.name,
            user.premium_type as _,
            user.public_flags as _,
            user.user_id,
            user.verified,
        )
        .fetch_one(&self.pool)
        .await
    }
}
