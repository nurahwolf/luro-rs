use twilight_model::{
    gateway::payload::incoming::{MemberAdd, MemberChunk, MemberRemove, MemberUpdate},
    guild::{Member, PartialMember},
    id::{marker::GuildMarker, Id},
};

pub enum MemberSync<'a> {
    Member(Id<GuildMarker>, &'a Member),
    MemberAdd(&'a MemberAdd),
    MemberChunk(&'a MemberChunk),
    MemberRemove(&'a MemberRemove),
    MemberUpdate(&'a MemberUpdate),
    PartialMember(Id<GuildMarker>, &'a PartialMember),
    // LuroMember(LuroMember)
}

impl<'a> From<(Id<GuildMarker>, &'a Member)> for MemberSync<'a> {
    fn from(data: (Id<GuildMarker>, &'a Member)) -> Self {
        Self::Member(data.0, data.1)
    }
}

impl<'a> From<(Id<GuildMarker>, &'a PartialMember)> for MemberSync<'a> {
    fn from(data: (Id<GuildMarker>, &'a PartialMember)) -> Self {
        Self::PartialMember(data.0, data.1)
    }
}

impl<'a> From<&'a MemberAdd> for MemberSync<'a> {
    fn from(data: &'a MemberAdd) -> Self {
        Self::MemberAdd(data)
    }
}

impl<'a> From<&'a MemberChunk> for MemberSync<'a> {
    fn from(data: &'a MemberChunk) -> Self {
        Self::MemberChunk(data)
    }
}

impl<'a> From<&'a MemberRemove> for MemberSync<'a> {
    fn from(data: &'a MemberRemove) -> Self {
        Self::MemberRemove(data)
    }
}

impl<'a> From<&'a MemberUpdate> for MemberSync<'a> {
    fn from(data: &'a MemberUpdate) -> Self {
        Self::MemberUpdate(data)
    }
}
