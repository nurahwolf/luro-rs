use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate, UserUpdate},
    guild::{Member, PartialMember},
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    user::CurrentUser,
};

use crate::user::User;

/// Used for handling incoming data types to be mapped to database actions
pub enum UserSync<'a> {
    User(&'a User),
    TwilightUser(&'a twilight_model::user::User),
    UserID(Id<UserMarker>),
    UserUpdate(&'a UserUpdate),
    CurrentUser(&'a CurrentUser),
    Member(Id<GuildMarker>, &'a Member),
    MemberAdd(&'a MemberAdd),
    MemberChunk(&'a MemberChunk),
    MemberRemove(&'a MemberRemove),
    MemberUpdate(&'a MemberUpdate),
    PartialMember(Id<GuildMarker>, &'a PartialMember),
}

impl<'a> From<&'a CurrentUser> for UserSync<'a> {
    fn from(user: &'a CurrentUser) -> Self {
        Self::CurrentUser(user)
    }
}

impl<'a> From<&'a UserUpdate> for UserSync<'a> {
    fn from(user: &'a UserUpdate) -> Self {
        Self::UserUpdate(user)
    }
}

impl<'a> From<&'a User> for UserSync<'a> {
    fn from(user: &'a User) -> Self {
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

impl<'a> From<(Id<GuildMarker>, &'a Member)> for UserSync<'a> {
    fn from(data: (Id<GuildMarker>, &'a Member)) -> Self {
        Self::Member(data.0, data.1)
    }
}

impl<'a> From<(Id<GuildMarker>, &'a PartialMember)> for UserSync<'a> {
    fn from(data: (Id<GuildMarker>, &'a PartialMember)) -> Self {
        Self::PartialMember(data.0, data.1)
    }
}

impl<'a> From<&'a MemberAdd> for UserSync<'a> {
    fn from(data: &'a MemberAdd) -> Self {
        Self::MemberAdd(data)
    }
}

impl<'a> From<&'a MemberChunk> for UserSync<'a> {
    fn from(data: &'a MemberChunk) -> Self {
        Self::MemberChunk(data)
    }
}

impl<'a> From<&'a MemberRemove> for UserSync<'a> {
    fn from(data: &'a MemberRemove) -> Self {
        Self::MemberRemove(data)
    }
}

impl<'a> From<&'a MemberUpdate> for UserSync<'a> {
    fn from(data: &'a MemberUpdate) -> Self {
        Self::MemberUpdate(data)
    }
}
