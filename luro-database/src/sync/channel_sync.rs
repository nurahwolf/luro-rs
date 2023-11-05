use twilight_model::{
    channel::Channel,
    gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate}, id::{marker::ChannelMarker, Id},
};

pub enum ChannelSync {
    ChannelID(Id<ChannelMarker>),
    Channel(Channel),
    ChannelCreate(Box<ChannelCreate>),
    ChannelDelete(Box<ChannelDelete>),
    ChannelUpdate(Box<ChannelUpdate>),
    ChannelPinsUpdate(ChannelPinsUpdate),
}

impl From<Id<ChannelMarker>> for ChannelSync {
    fn from(value: Id<ChannelMarker>) -> Self {
        Self::ChannelID(value)
    }
}

impl From<Box<ChannelCreate>> for ChannelSync {
    fn from(channel: Box<ChannelCreate>) -> Self {
        Self::ChannelCreate(channel)
    }
}

impl From<Box<ChannelDelete>> for ChannelSync {
    fn from(channel: Box<ChannelDelete>) -> Self {
        Self::ChannelDelete(channel)
    }
}

impl From<Box<ChannelUpdate>> for ChannelSync {
    fn from(channel: Box<ChannelUpdate>) -> Self {
        Self::ChannelUpdate(channel)
    }
}

impl From<ChannelPinsUpdate> for ChannelSync {
    fn from(channel: ChannelPinsUpdate) -> Self {
        Self::ChannelPinsUpdate(channel)
    }
}

impl From<Channel> for ChannelSync {
    fn from(channel: Channel) -> Self {
        Self::Channel(channel)
    }
}
