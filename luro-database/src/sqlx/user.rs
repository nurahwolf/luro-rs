use sqlx::types::Json;
use twilight_model::gateway::payload::incoming::UserUpdate;
use twilight_model::user::PremiumType;
use twilight_model::user::User;
use twilight_model::user::UserFlags;
use twilight_model::util::ImageHash;

use luro_model::user::LuroUser;
use twilight_model::id::Id;

mod count_users;
mod get_staff;
mod get_user;
mod get_users;
mod handle_luro_user;
mod handle_user;
mod handle_user_update;
mod update_user;

#[derive(Debug, Default, ::sqlx::Type)]
#[sqlx(type_name = "user_permissions", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuroUserPermissions {
    #[default]
    User,
    Owner,
    Administrator,
}

impl From<luro_model::user::LuroUserPermissions> for LuroUserPermissions {
    fn from(perms: luro_model::user::LuroUserPermissions) -> Self {
        match perms {
            luro_model::user::LuroUserPermissions::User => Self::User,
            luro_model::user::LuroUserPermissions::Owner => Self::Owner,
            luro_model::user::LuroUserPermissions::Administrator => Self::Administrator,
        }
    }
}

impl From<LuroUserPermissions> for luro_model::user::LuroUserPermissions {
    fn from(perms: LuroUserPermissions) -> Self {
        match perms {
            LuroUserPermissions::User => Self::User,
            LuroUserPermissions::Owner => Self::Owner,
            LuroUserPermissions::Administrator => Self::Administrator,
        }
    }
}

pub enum DatabaseUserType {
    User(User),
    UserUpdate(UserUpdate),
    LuroUser(LuroUser),
}

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
            accent_color: user.accent_colour.map(|x| x as u32),
            avatar_decoration: user.avatar_decoration.map(|x| x.0),
            avatar: user.avatar.map(|x| x.0),
            averagesize: user.words_average.map(|x| x as usize).unwrap_or_default(),
            banner: user.banner.map(|x| x.0),
            bot: user.bot.unwrap_or_default(),
            character_prefix: Default::default(),
            characters: Default::default(),
            discriminator: user.discriminator as u16,
            email: user.email,
            flags: user.flags.map(|x| x.0),
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
            premium_type: user.premium_type.map(|x| x.0),
            public_flags: user.public_flags.map(|x| x.0),
            system: user.system.unwrap_or_default(),
            user_permissions: user.user_permissions.unwrap_or_default().into(),
            verified: user.verified.unwrap_or_default(),
            warnings: Default::default(),
            wordcount: user.words_count.map(|x| x as usize).unwrap_or_default(),
            words: Default::default(),
            wordsize: Default::default(),
        }
    }
}
