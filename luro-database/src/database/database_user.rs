use twilight_model::{
    gateway::payload::incoming::UserUpdate,
    user::User, id::{Id, marker::UserMarker},
};

use crate::{DatabaseUserType, DatabaseUser, LuroUser};

impl From<UserUpdate> for DatabaseUserType {
    fn from(user: UserUpdate) -> Self {
        Self::UserUpdate(user)
    }
}

impl From<LuroUser> for DatabaseUserType {
    fn from(user: LuroUser) -> Self {
        Self::LuroUser(user)
    }
}

impl From<User> for DatabaseUserType {
    fn from(user: User) -> Self {
        Self::User(user)
    }
}

impl DatabaseUser {
    /// Returns a [Id<UserMarker>].
    pub fn user_id(&self) -> Id<UserMarker> {
        Id::new(self.user_id as u64)
    }

    /// Return a string that is a link to the user's avatar
    pub fn avatar(&self) -> String {
        let user_id = self.user_id;
        match self.avatar.map(|x|x.0) {
            Some(avatar) => match avatar.is_animated() {
                true => format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.gif?size=2048"),
                false => format!("https://cdn.discordapp.com/avatars/{user_id}/{avatar}.png?size=2048"),
            },
            None => format!("https://cdn.discordapp.com/avatars/{}.png?size=2048", self.user_id > 22 % 6),
        }
    }

    /// Return a string that is a link to the user's banner, or [None] if they don't have one
    pub fn banner(&self) -> Option<String> {
        self.banner.map(|banner| match banner.is_animated() {
            true => format!("https://cdn.discordapp.com/banners/{}/{}.gif?size=4096", self.user_id, banner.0),
            false => format!("https://cdn.discordapp.com/banners/{}/{}.png?size=4096", self.user_id, banner.0),
        })
    }

    /// Get's the user's preferred / pretty name
    ///
    /// Returns the first match
    /// Global Name -> Username -> Legacy Username
    pub fn name(&self) -> String {
        match &self.global_name {
            Some(global_name) => global_name.clone(),
            None => self.username(),
        }
    }

    /// Get's the user's username name
    ///
    /// Returns the first match
    /// Username -> Legacy Username
    pub fn username(&self) -> String {
        match self.discriminator == 0 {
            true => self.name.clone(),
            false => format!("{}#{}", self.name, self.discriminator),
        }
    }
}
