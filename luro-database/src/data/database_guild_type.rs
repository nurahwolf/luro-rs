use luro_model::guild::LuroGuild;
use twilight_model::{gateway::payload::incoming::GuildUpdate, guild::Guild};

use crate::DatabaseGuildType;

impl From<Box<GuildUpdate>> for DatabaseGuildType {
    fn from(guild: Box<GuildUpdate>) -> Self {
        Self::GuildUpdate(guild)
    }
}

impl From<LuroGuild> for DatabaseGuildType {
    fn from(guild: LuroGuild) -> Self {
        Self::LuroGuild(guild)
    }
}

impl From<Guild> for DatabaseGuildType {
    fn from(guild: Guild) -> Self {
        Self::Guild(guild)
    }
}