use twilight_model::{
    gateway::payload::incoming::UserUpdate,
    id::{marker::UserMarker, Id},
    user::CurrentUser,
};

use crate::types::User;

/// Used for handling incoming data types to be mapped to database actions
pub enum UserSync<'a> {
    User(User),
    TwilightUser(&'a twilight_model::user::User),
    UserID(Id<UserMarker>),
    UserUpdate(UserUpdate),
    CurrentUser(&'a CurrentUser),
}

impl<'a> From<&'a CurrentUser> for UserSync<'a> {
    fn from(user: &'a CurrentUser) -> Self {
        Self::CurrentUser(user)
    }
}

impl<'a> From<UserUpdate> for UserSync<'a> {
    fn from(user: UserUpdate) -> Self {
        Self::UserUpdate(user)
    }
}

impl<'a> From<User> for UserSync<'a> {
    fn from(user: User) -> Self {
        Self::User(user)
    }
}

impl<'a> From<&'a twilight_model::user::User> for UserSync<'a> {
    fn from(user: &'a twilight_model::user::User) -> Self {
        Self::TwilightUser(user)
    }
}

impl<'a> From<&'a twilight_model::guild::Member> for UserSync<'a> {
    fn from(member: &'a twilight_model::guild::Member) -> Self {
        Self::TwilightUser(&member.user)
    }
}

impl<'a> From<Id<UserMarker>> for UserSync<'a> {
    fn from(user: Id<UserMarker>) -> Self {
        Self::UserID(user)
    }
}
