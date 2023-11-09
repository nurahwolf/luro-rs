use twilight_model::{gateway::payload::incoming::GuildUpdate, guild::Guild};
pub enum GuildSync<'a> {
    Guild(&'a Guild),
    GuildUpdate(&'a GuildUpdate),
}

impl<'a> From<&'a GuildUpdate> for GuildSync<'a> {
    fn from(guild: &'a GuildUpdate) -> Self {
        Self::GuildUpdate(guild)
    }
}

impl<'a> From<&'a Guild> for GuildSync<'a> {
    fn from(guild: &'a Guild) -> Self {
        Self::Guild(guild)
    }
}
