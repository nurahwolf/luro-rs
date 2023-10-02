use luro_model::user::LuroUser;
use twilight_model::{
    gateway::payload::incoming::UserUpdate,
    id::{marker::UserMarker, Id},
    user::User,
};

use crate::{DatabaseUser, DatabaseUserType, LuroUserPermissions};

impl From<DatabaseUserType> for DatabaseUser {
    fn from(user: DatabaseUserType) -> Self {
        match user {
            DatabaseUserType::User(user) => Self {
                accent_colour: user.accent_color.map(|x| x as i32),
                user_id: user.id.get() as i64,
                name: user.name,
                user_permissions: LuroUserPermissions::default(),
            },
            DatabaseUserType::UserUpdate(user) => Self {
                accent_colour: user.accent_color.map(|x| x as i32),
                user_id: user.id.get() as i64,
                name: user.name.clone(),
                user_permissions: LuroUserPermissions::default(),
            },
            DatabaseUserType::LuroUser(user) => Self {
                accent_colour: user.accent_color.map(|x| x as i32),
                user_id: user.id.get() as i64,
                name: user.name,
                user_permissions: LuroUserPermissions::default(),
            },
        }
    }
}

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
    /// Return's a Twilight [Id<UserMarker>]
    pub fn user_id(&self) -> Id<UserMarker> {
        Id::new(self.user_id as u64)
    }
}
