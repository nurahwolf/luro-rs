use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate},
    guild::{Member, PartialMember},
    id::{
        marker::GuildMarker,
        Id,
    },
};

use crate::DbMemberType;

impl From<(Id<GuildMarker>, Member)> for DbMemberType {
    fn from(data: (Id<GuildMarker>, Member)) -> Self {
        Self::Member(data.0, data.1)
    }
}

impl From<(Id<GuildMarker>, PartialMember)> for DbMemberType {
    fn from(data: (Id<GuildMarker>, PartialMember)) -> Self {
        Self::PartialMember(data.0, data.1)
    }
}

impl From<Box<MemberAdd>> for DbMemberType {
    fn from(data: Box<MemberAdd>) -> Self {
        Self::MemberAdd(data)
    }
}

impl From<MemberChunk> for DbMemberType {
    fn from(data: MemberChunk) -> Self {
        Self::MemberChunk(data)
    }
}

impl From<MemberRemove> for DbMemberType {
    fn from(data: MemberRemove) -> Self {
        Self::MemberRemove(data)
    }
}

impl From<Box<MemberUpdate>> for DbMemberType {
    fn from(data: Box<MemberUpdate>) -> Self {
        Self::MemberUpdate(data)
    }
}
