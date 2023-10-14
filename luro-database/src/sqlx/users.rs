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

#[derive(Debug)]
pub struct DatabaseUser {
    // pub character_prefix: BTreeMap<String, String>,
    // pub guilds: HashMap<Id<GuildMarker>, LuroMember>,
    // pub marriages: BTreeMap<Id<UserMarker>, UserMarriages>,
    // pub moderation_actions_performed: usize,
    // pub moderation_actions: Json<Vec<UserActions>>,
    // pub words: Json<BTreeMap<String, usize>>,
    // pub wordsize: Json<BTreeMap<usize, usize>>,
    pub accent_colour: Option<i32>,
    pub avatar_decoration: Option<Json<ImageHash>>,
    pub avatar: Option<Json<ImageHash>>,
    pub banner: Option<Json<ImageHash>>,
    pub bot: Option<bool>,
    pub characters: Option<Vec<i32>>,
    pub discriminator: i16,
    pub email: Option<String>,
    pub flags: Option<Json<UserFlags>>,
    pub global_name: Option<String>,
    pub locale: Option<String>,
    pub message_edits: Option<i64>,
    pub messages: Option<Vec<i64>>,
    pub mfa_enabled: Option<bool>,
    pub name: String,
    pub premium_type: Option<Json<PremiumType>>,
    pub public_flags: Option<Json<UserFlags>>,
    pub system: Option<bool>,
    pub user_id: i64,
    pub user_permissions: Option<LuroUserPermissions>,
    pub verified: Option<bool>,
    pub warnings: Option<Vec<i64>>,
    pub words_average: Option<i64>,
    pub words_count: Option<i64>,
}

impl From<DatabaseUser> for LuroUser {
    fn from(user: DatabaseUser) -> Self {
        Self {
            accent_color: user.accent_colour.map(|x|x as u32),
            avatar_decoration: user.avatar_decoration.map(|x|x.0),
            avatar: user.avatar.map(|x|x.0),
            averagesize: user.words_average.map(|x|x as usize).unwrap_or_default(),
            banner: user.banner.map(|x|x.0),
            bot: user.bot.unwrap_or_default(),
            character_prefix: Default::default(),
            characters: Default::default(),
            discriminator: user.discriminator as u16,
            email: user.email,
            flags: user.flags.map(|x|x.0),
            global_name: user.global_name,
            guilds: Default::default(),
            id: Id::new(user.user_id as u64),
            locale: user.locale,
            marriages: Default::default(),
            message_edits: Default::default(),
            messages: Default::default(),
            mfa_enabled: user.mfa_enabled.unwrap_or_default(),
            moderation_actions_performed: Default::default(),
            moderation_actions: Default::default(),
            name: user.name,
            premium_type: user.premium_type.map(|x|x.0),
            public_flags: user.public_flags.map(|x|x.0),
            system: user.system.unwrap_or_default(),
            user_permissions: user.user_permissions.unwrap_or_default().into(),
            verified: user.verified.unwrap_or_default(),
            warnings: Default::default(),
            wordcount: user.words_count.map(|x|x as usize).unwrap_or_default(),
            words: Default::default(),
            wordsize: Default::default(),
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
