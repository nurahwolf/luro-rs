use twilight_model::{
    gateway::payload::incoming::UserUpdate,
    id::{marker::UserMarker, Id},
    user::User,
};

/// Used for handling incoming data types to be mapped to database actions
pub enum UserSync {
    User(User),
    UserID(Id<UserMarker>),
    UserUpdate(UserUpdate),
}

impl From<UserUpdate> for UserSync {
    fn from(user: UserUpdate) -> Self {
        Self::UserUpdate(user)
    }
}

impl From<User> for UserSync {
    fn from(user: User) -> Self {
        Self::User(user)
    }
}

impl From<Id<UserMarker>> for UserSync {
    fn from(user: Id<UserMarker>) -> Self {
        Self::UserID(user)
    }
}