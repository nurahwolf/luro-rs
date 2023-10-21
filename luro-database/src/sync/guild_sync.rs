use twilight_model::{gateway::payload::incoming::GuildUpdate, guild::Guild};

use crate::LuroGuild;

pub enum GuildSync {
    Guild(Guild),
    GuildUpdate(Box<GuildUpdate>),
    LuroGuild(LuroGuild),
}

impl From<Box<GuildUpdate>> for GuildSync {
    fn from(guild: Box<GuildUpdate>) -> Self {
        Self::GuildUpdate(guild)
    }
}

impl From<LuroGuild> for GuildSync {
    fn from(guild: LuroGuild) -> Self {
        Self::LuroGuild(guild)
    }
}

impl From<Guild> for GuildSync {
    fn from(guild: Guild) -> Self {
        Self::Guild(guild)
    }
}
