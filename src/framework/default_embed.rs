use twilight_model::id::{marker::GuildMarker, Id};
use twilight_util::builder::embed::EmbedBuilder;

use super::LuroFramework;

impl LuroFramework {
    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    pub fn default_embed(&self, guild_id: &Option<Id<GuildMarker>>) -> EmbedBuilder {
        EmbedBuilder::new().color(self.accent_colour(guild_id))
    }
}
