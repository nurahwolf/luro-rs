use twilight_model::{
    gateway::payload::incoming::{GuildCreate, GuildUpdate},
    guild::{Guild, UnavailableGuild},
};
pub enum GuildSync<'a> {
    Guild(&'a Guild),
    GuildUpdate(&'a GuildUpdate),
    GuildCreate(&'a GuildCreate),
    GuildUnavailable(&'a UnavailableGuild),
}

impl<'a> From<&'a UnavailableGuild> for GuildSync<'a> {
    fn from(guild: &'a UnavailableGuild) -> Self {
        Self::GuildUnavailable(guild)
    }
}

impl<'a> From<&'a GuildUpdate> for GuildSync<'a> {
    fn from(guild: &'a GuildUpdate) -> Self {
        Self::GuildUpdate(guild)
    }
}

impl<'a> From<&'a GuildCreate> for GuildSync<'a> {
    fn from(guild: &'a GuildCreate) -> Self {
        Self::GuildCreate(guild)
    }
}

impl<'a> From<&'a Guild> for GuildSync<'a> {
    fn from(guild: &'a Guild) -> Self {
        Self::Guild(guild)
    }
}
