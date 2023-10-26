use serde::{Deserialize, Serialize};
use twilight_model::gateway::payload::incoming::UserUpdate;

use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;
use twilight_model::user::User;

mod count_characters;
mod count_moderation_actions;
mod count_users;
mod count_warnings;
mod get_staff;
mod get_user;
mod get_users;
mod update_user;
mod update_user_data;
mod update_user_permissions;

#[derive(Debug, Default, Clone, ::sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_permissions", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuroUserPermissions {
    Administrator,
    Owner,
    #[default]
    User,
}

impl std::fmt::Display for LuroUserPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LuroUserPermissions::Administrator => write!(f, "ADMINISTRATOR"),
            LuroUserPermissions::Owner => write!(f, "OWNER"),
            LuroUserPermissions::User => write!(f, "USER"),
        }
    }
}
pub enum DatabaseUserType {
    User(User),
    UserID(Id<UserMarker>),
    UserUpdate(UserUpdate),
}

#[derive(Clone, Debug)]
pub struct DatabaseUser {
    pub accent_colour: Option<i32>,
    pub avatar_decoration: Option<String>,
    pub user_avatar: Option<String>,
    pub user_banner: Option<String>,
    pub bot: bool,
    pub characters: Option<Vec<i32>>,
    pub discriminator: i16,
    pub email: Option<String>,
    pub user_flags: Option<i64>,
    pub global_name: Option<String>,
    pub locale: Option<String>,
    pub message_edits: Option<i64>,
    pub messages: Option<Vec<i64>>,
    pub mfa_enabled: Option<bool>,
    pub user_name: String,
    pub premium_type: Option<i16>,
    pub public_flags: Option<i64>,
    pub user_system: Option<bool>,
    pub user_id: i64,
    pub user_permissions: LuroUserPermissions,
    pub verified: Option<bool>,
    pub warnings: Option<Vec<i64>>,
    pub words_average: Option<i64>,
    pub words_count: Option<i64>,
}
