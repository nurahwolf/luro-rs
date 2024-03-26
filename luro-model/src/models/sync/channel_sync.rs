use twilight_model::{
    channel::Channel,
    gateway::payload::incoming::{ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate},
    id::{marker::ChannelMarker, Id},
};

pub enum ChannelSync<'a> {
    ChannelID(Id<ChannelMarker>),
    Channel(&'a Channel),
    ChannelCreate(&'a ChannelCreate),
    ChannelDelete(&'a ChannelDelete),
    ChannelUpdate(&'a ChannelUpdate),
    ChannelPinsUpdate(&'a ChannelPinsUpdate),
}

impl<'a> From<Id<ChannelMarker>> for ChannelSync<'a> {
    fn from(value: Id<ChannelMarker>) -> Self {
        Self::ChannelID(value)
    }
}

impl<'a> From<&'a ChannelCreate> for ChannelSync<'a> {
    fn from(channel: &'a ChannelCreate) -> Self {
        Self::ChannelCreate(channel)
    }
}

impl<'a> From<&'a ChannelDelete> for ChannelSync<'a> {
    fn from(channel: &'a ChannelDelete) -> Self {
        Self::ChannelDelete(channel)
    }
}

impl<'a> From<&'a ChannelUpdate> for ChannelSync<'a> {
    fn from(channel: &'a ChannelUpdate) -> Self {
        Self::ChannelUpdate(channel)
    }
}

impl<'a> From<&'a ChannelPinsUpdate> for ChannelSync<'a> {
    fn from(channel: &'a ChannelPinsUpdate) -> Self {
        Self::ChannelPinsUpdate(channel)
    }
}

impl<'a> From<&'a Channel> for ChannelSync<'a> {
    fn from(channel: &'a Channel) -> Self {
        Self::Channel(channel)
    }
}
