use twilight_model::{
    channel::Channel,
    gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate},
};

mod count_channels;
mod update_channel;

pub struct DbChannel {
    pub channel_id: i64,
    pub deleted: bool,
    pub guild_id: Option<i64>,
}

pub enum DbChannelType {
    DbChannel(DbChannel),
    Channel(Channel),
    ChannelCreate(Box<ChannelCreate>),
    ChannelDelete(Box<ChannelDelete>),
    ChannelUpdate(Box<ChannelUpdate>),
    ChannelPinsUpdate(ChannelPinsUpdate),
}

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
