use twilight_model::{
    channel::Channel,
    gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate},
};

use crate::DbChannelType;

impl From<Box<ChannelCreate>> for DbChannelType {
    fn from(channel: Box<ChannelCreate>) -> Self {
        Self::ChannelCreate(channel)
    }
}

impl From<Box<ChannelDelete>> for DbChannelType {
    fn from(channel: Box<ChannelDelete>) -> Self {
        Self::ChannelDelete(channel)
    }
}

impl From<Box<ChannelUpdate>> for DbChannelType {
    fn from(channel: Box<ChannelUpdate>) -> Self {
        Self::ChannelUpdate(channel)
    }
}

impl From<ChannelPinsUpdate> for DbChannelType {
    fn from(channel: ChannelPinsUpdate) -> Self {
        Self::ChannelPinsUpdate(channel)
    }
}

impl From<Channel> for DbChannelType {
    fn from(channel: Channel) -> Self {
        Self::Channel(channel)
    }
}
