use twilight_model::{gateway::payload::incoming::UserUpdate, user::User, id::{marker::UserMarker, Id}};

use crate::DatabaseUserType;

impl From<UserUpdate> for DatabaseUserType {
    fn from(user: UserUpdate) -> Self {
        Self::UserUpdate(user)
    }
}

impl From<User> for DatabaseUserType {
    fn from(user: User) -> Self {
        Self::User(user)
    }
}

impl From<Id<UserMarker>> for DatabaseUserType {
    fn from(user: Id<UserMarker>) -> Self {
        Self::UserID(user)
    }
}
