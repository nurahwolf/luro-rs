
use serde::{Serialize, Deserialize};
use twilight_model::gateway::payload::incoming::UserUpdate;

use twilight_model::user::User;



use crate::LuroUser;

mod count_characters;
mod count_moderation_actions;
mod count_users;
mod count_warnings;
mod get_staff;
mod get_user;
mod get_users;
mod handle_luro_user;
mod handle_user;
mod handle_user_update;
mod update_user;
mod update_user_data;

#[derive(Debug, Default, Clone, ::sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "user_permissions", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuroUserPermissions {
    #[default]
    User,
    Owner,
    Administrator,
}

pub enum DatabaseUserType {
    User(User),
    UserUpdate(UserUpdate),
    LuroUser(LuroUser),
}

#[derive(Clone, Debug)]
pub struct DatabaseUser {
    pub accent_colour: Option<i32>,
    pub avatar_decoration: Option<String>,
    pub user_avatar: Option<String>,
    pub banner: Option<String>,
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
    pub name: String,
    pub premium_type: Option<i16>,
    pub public_flags: Option<i64>,
    pub system: Option<bool>,
    pub user_id: i64,
    pub user_permissions: LuroUserPermissions,
    pub verified: Option<bool>,
    pub warnings: Option<Vec<i64>>,
    pub words_average: Option<i64>,
    pub words_count: Option<i64>,
}
