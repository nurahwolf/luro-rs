use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate},
    guild::{Member, PartialMember},
    id::{marker::GuildMarker, Id},
};

pub enum MemberSync {
    Member(Id<GuildMarker>, Member),
    MemberAdd(Box<MemberAdd>),
    MemberChunk(MemberChunk),
    MemberRemove(MemberRemove),
    MemberUpdate(Box<MemberUpdate>),
    PartialMember(Id<GuildMarker>, PartialMember),
    // LuroMember(LuroMember)
}

impl From<(Id<GuildMarker>, Member)> for MemberSync {
    fn from(data: (Id<GuildMarker>, Member)) -> Self {
        Self::Member(data.0, data.1)
    }
}

impl From<(Id<GuildMarker>, PartialMember)> for MemberSync {
    fn from(data: (Id<GuildMarker>, PartialMember)) -> Self {
        Self::PartialMember(data.0, data.1)
    }
}

impl From<Box<MemberAdd>> for MemberSync {
    fn from(data: Box<MemberAdd>) -> Self {
        Self::MemberAdd(data)
    }
}

impl From<MemberChunk> for MemberSync {
    fn from(data: MemberChunk) -> Self {
        Self::MemberChunk(data)
    }
}

impl From<MemberRemove> for MemberSync {
    fn from(data: MemberRemove) -> Self {
        Self::MemberRemove(data)
    }
}

impl From<Box<MemberUpdate>> for MemberSync {
    fn from(data: Box<MemberUpdate>) -> Self {
        Self::MemberUpdate(data)
    }
}
