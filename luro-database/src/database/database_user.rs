use twilight_model::{gateway::payload::incoming::UserUpdate, user::User};

use crate::{DatabaseUserType, LuroUser};

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
