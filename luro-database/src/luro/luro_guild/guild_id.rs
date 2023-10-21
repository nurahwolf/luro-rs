use twilight_model::id::{marker::GuildMarker, Id};

use crate::LuroGuild;

impl LuroGuild {
    pub fn guild_id(&self) -> Id<GuildMarker> {
        Id::new(self.guild_id as u64)
    }
}
