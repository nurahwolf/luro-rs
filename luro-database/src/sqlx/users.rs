use sqlx::types::Json;
use twilight_model::util::ImageHash;
use twilight_model::user::UserFlags;
use twilight_model::user::PremiumType;

use luro_model::user::LuroUser;
use sqlx::Error;
use twilight_model::id::Id;

use crate::{DatabaseUser, LuroDatabase, LuroUserPermissions};

mod count_users;
mod get_user;
mod get_users;
mod handle_luro_user;
mod handle_user;
mod handle_user_update;
mod update_user;

impl From<DatabaseUser> for LuroUser {
    fn from(user: DatabaseUser) -> Self {
        Self {
            accent_color: user.accent_colour.map(|x|x as u32),
            avatar: user.avatar.map(|x|x.0),
            avatar_decoration: user.avatar_decoration.map(|x|x.0),
            banner: user.banner.map(|x|x.0),
            bot: user.bot,
            discriminator: user.discriminator as u16,
            global_name: user.global_name,
            email: user.email,
            flags: user.flags.map(|x|x.0),
            id: Id::new(user.user_id as u64),
            locale: user.locale,
            mfa_enabled: user.mfa_enabled,
            name: user.name,
            premium_type: user.premium_type.map(|x|x.0),
            public_flags: user.public_flags.map(|x|x.0),
            system: user.system,
            verified: user.verified,
            wordcount: user.words_count.map(|x|x as usize).unwrap_or_default(),
            averagesize: user.words_average.map(|x|x as usize).unwrap_or_default(),
            wordsize: Default::default(),
            words: Default::default(),
            warnings: Default::default(),
            messages: Default::default(),
            moderation_actions: Default::default(),
            moderation_actions_performed: Default::default(),
            message_edits: Default::default(),
            marriages: Default::default(),
            guilds: Default::default(),
            characters: Default::default(),
            character_prefix: Default::default(),
            user_permissions: user.user_permissions.into()
        }
    }
}

impl LuroDatabase {
    pub async fn get_staff(&self) -> Result<Vec<DatabaseUser>, Error> {
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
        ).fetch_all(&self.0).await

    }
}
