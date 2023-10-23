use sqlx::Error;
use twilight_model::gateway::payload::incoming::UserUpdate;

use crate::{DatabaseUser, LuroDatabase, LuroUserPermissions};

impl LuroDatabase {
    pub async fn handle_user_update(&self, user: UserUpdate) -> Result<DatabaseUser, Error> {
        sqlx::query_as!(
            DatabaseUser,
            "INSERT INTO users (
                accent_colour,
                user_avatar,
                user_banner,
                bot,
                discriminator,
                email,
                user_flags,
                locale,
                mfa_enabled,
                user_name,
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
                user_avatar = $2,
                user_banner = $3,
                bot = $4,
                discriminator = $5,
                email = $6,
                user_flags = $7,
                locale = $8,
                mfa_enabled = $9,
                user_name = $10,
                premium_type = $11,
                public_flags = $12,
                verified = $14
            RETURNING
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
            ",
            user.accent_color.map(|x| x as i32),
            user.avatar.map(|x|x.to_string()),
            user.banner.map(|x|x.to_string()),
            user.bot,
            user.discriminator as i16,
            user.email,
            user.flags.map(|x|x.bits()as i64),
            user.locale,
            user.mfa_enabled,
            user.name,
            user.premium_type.map(|x| u8::from(x) as i16),
            user.public_flags.map(|x|x.bits() as i64),
            user.id.get() as i64,
            user.verified,
        )
        .fetch_one(&self.pool)
        .await
    }
}
