use luro_model::user::LuroUser;
use twilight_model::{
    gateway::payload::incoming::UserUpdate,
    id::{marker::UserMarker, Id},
    user::User,
};

use crate::{DatabaseUser, DatabaseUserType};

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
