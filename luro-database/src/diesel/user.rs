use diesel::{Queryable, Selectable};
use twilight_model::{
    user::{PremiumType, UserFlags},
    util::ImageHash,
};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DatabaseUser {
    pub accent_colour: Option<i32>,
    pub avatar_decoration: Option<ImageHash>,
    pub avatar: Option<ImageHash>,
    pub banner: Option<ImageHash>,
    pub bot: Option<bool>,
    pub characters: Option<Vec<i32>>,
    pub discriminator: i16,
    pub email: Option<String>,
    pub flags: Option<UserFlags>,
    pub global_name: Option<String>,
    pub locale: Option<String>,
    pub message_edits: Option<i64>,
    pub messages: Option<Vec<i64>>,
    pub mfa_enabled: Option<bool>,
    pub name: String,
    pub premium_type: Option<PremiumType>,
    pub public_flags: Option<UserFlags>,
    pub system: Option<bool>,
    pub user_id: i64,
    pub user_permissions: Option<LuroUserPermissions>,
    pub verified: Option<bool>,
    pub warnings: Option<Vec<i64>>,
    pub words_average: Option<i64>,
    pub words_count: Option<i64>,
}

#[derive(Debug, Default)]
pub enum LuroUserPermissions {
    #[default]
    User,
    Owner,
    Administrator,
}

impl From<LuroUserPermissions> for luro_model::user::LuroUserPermissions {
    fn from(permissions: LuroUserPermissions) -> Self {
        match permissions {
            LuroUserPermissions::User => Self::User,
            LuroUserPermissions::Owner => Self::Owner,
            LuroUserPermissions::Administrator => Self::Administrator,
        }
    }
}

impl From<luro_model::user::LuroUserPermissions> for LuroUserPermissions {
    fn from(permissions: luro_model::user::LuroUserPermissions) -> Self {
        match permissions {
            luro_model::user::LuroUserPermissions::User => Self::User,
            luro_model::user::LuroUserPermissions::Owner => Self::Owner,
            luro_model::user::LuroUserPermissions::Administrator => Self::Administrator,
        }
    }
}
