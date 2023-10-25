use twilight_model::{gateway::payload::incoming::GuildUpdate, guild::Guild};
pub enum GuildSync {
    Guild(Guild),
    GuildUpdate(Box<GuildUpdate>),
}

impl From<Box<GuildUpdate>> for GuildSync {
    fn from(guild: Box<GuildUpdate>) -> Self {
        Self::GuildUpdate(guild)
    }
}

impl From<Guild> for GuildSync {
    fn from(guild: Guild) -> Self {
        Self::Guild(guild)
    }
}
