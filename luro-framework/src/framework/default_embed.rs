use luro_builder::embed::EmbedBuilder;
use luro_model::{database::drivers::LuroDatabaseDriver, ACCENT_COLOUR};
use tracing::error;
use twilight_model::id::{marker::GuildMarker, Id};

use crate::Framework;

impl<D: LuroDatabaseDriver> Framework<D> {
    /// Create a default embed, with the guild's accent colour if present.
    ///
    /// Will fall back to the static [ACCENT_COLOUR] if an error occured
    pub async fn default_embed(&self, guild_id: Option<&Id<GuildMarker>>) -> EmbedBuilder {
        let mut embed = luro_builder::embed::EmbedBuilder::default();
        let colour = match guild_id {
            Some(guild_id) => match self.guild_accent_colour(guild_id).await {
                Ok(guild_colour) => guild_colour.unwrap_or(ACCENT_COLOUR),
                Err(why) => {
                    error!(why = ?why, "Failed to get the guild accent colour for guild {guild_id}");
                    ACCENT_COLOUR
                }
            },
            None => ACCENT_COLOUR
        };
        embed.colour(colour);
        embed
    }
}
