use luro_builder::embed::EmbedBuilder;
use twilight_model::id::{Id, marker::GuildMarker};

use crate::database::drivers::LuroDatabaseDriver;

use super::Context;

impl<D: LuroDatabaseDriver> Context<D> {
    /// Create a default embed which has the guild's accent colour if available, otherwise falls back to Luro's accent colour
    pub async fn default_embed(&self, guild_id: Option<&Id<GuildMarker>>) -> EmbedBuilder {
        EmbedBuilder::default().colour(self.accent_colour(guild_id).await).clone()
    }
}