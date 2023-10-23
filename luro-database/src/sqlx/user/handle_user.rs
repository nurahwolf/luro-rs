use sqlx::types::Json;
use sqlx::Error;

use twilight_model::user::User;

use crate::{DatabaseUser, LuroDatabase, LuroUserPermissions};

impl LuroDatabase {
    pub async fn handle_user(&self, user: User) -> Result<DatabaseUser, Error> {
        sqlx::query_as!(
            DatabaseUser,
            "INSERT INTO users (
                accent_colour,
                user_avatar,
                avatar_decoration,
                banner,
                bot,
                discriminator,
                email,
                user_flags,
                global_name,
                locale,
                mfa_enabled,
                name,
                premium_type,
                public_flags,
                system,
                user_id,
                verified
            ) VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            ON CONFLICT
                (user_id)
            DO UPDATE SET
                accent_colour = $1,
                user_avatar = $2,
                avatar_decoration = $3,
                banner = $4,
                bot = $5,
                discriminator = $6,
                email = $7,
                user_flags = $8,
                global_name = $9,
                locale = $10,
                mfa_enabled = $11,
                name = $12,
                premium_type = $13,
                public_flags = $14,
                system = $15,
                verified = $17
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
            user.accent_color.map(|x| x as i32),
            user.avatar.map(|x|x.to_string()),
            user.avatar_decoration.map(|x|x.to_string()),
            user.banner.map(|x|x.to_string()),
            user.bot,
            user.discriminator as i16,
            user.email,
            user.flags.map(|x| x.bits() as i64),
            user.global_name,
            user.locale,
            user.mfa_enabled,
            user.name,
            user.premium_type.map(|x| u8::from(x) as i16),
            user.public_flags.map(|x|x.bits() as i64),
            user.system,
            user.id.get() as i64,
            user.verified,
        )
        .fetch_one(&self.pool)
        .await
    }
}
