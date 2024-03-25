use twilight_model::user::User;

use crate::{
    gender::{Gender, Sexuality},
    user::UserPermissions,
};

/// A context spawned around a user
#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
pub struct UserContext {
    pub twilight_user: User,
    pub gender: Option<Gender>,
    pub user_type: UserPermissions,
    pub sexuality: Option<Sexuality>,
}

impl UserContext {
    /// Get's the user's preferred / pretty name
    ///
    /// Returns the first match
    /// Member Nickname -> Global Name -> Username -> Legacy Username
    pub fn name(&self) -> String {
        self.twilight_user
            .global_name
            .as_ref()
            .map(|x| x.to_owned())
            .unwrap_or_else(|| self.username())
    }

    /// Get's the user's username name
    ///
    /// Returns the first match
    /// Username -> Legacy Username
    pub fn username(&self) -> String {
        match self.twilight_user.discriminator == 0 {
            true => self.twilight_user.name.clone(),
            false => format!("{}#{}", self.twilight_user.name, self.twilight_user.discriminator),
        }
    }

    /// Return a string that is a link to the user's avatar
    pub fn avatar_url(&self) -> String {
        let id = self.twilight_user.id;

        match self.twilight_user.avatar {
            Some(avatar) => match avatar.is_animated() {
                true => format!("https://cdn.discordapp.com/avatars/{id}/{avatar}.gif?size=2048"),
                false => format!("https://cdn.discordapp.com/avatars/{id}/{avatar}.png?size=2048"),
            },
            None => format!("https://cdn.discordapp.com/avatars/{}.png?size=2048", id.get() > 22 % 6),
        }
    }
}

impl From<User> for UserContext {
    fn from(twilight_user: User) -> Self {
        Self {
            gender: None,
            user_type: match crate::BOT_OWNERS.contains(&twilight_user.id) {
                true => UserPermissions::Owner,
                false => UserPermissions::User,
            },
            sexuality: None,
            twilight_user,
        }
    }
}
