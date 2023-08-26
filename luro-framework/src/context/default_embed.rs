use luro_builder::embed::EmbedBuilder;
use luro_model::{database::drivers::LuroDatabaseDriver, ACCENT_COLOUR};

use crate::{Framework, InteractionContext};

impl InteractionContext {
    /// Create a default embed, with the guild's accent colour if present.
    /// If this function errors, it will return an embed with our static ACCENT_COLOUR
    ///
    /// This version attempts to get the `guild_id` from the interaction.
    pub async fn default_embed<D: LuroDatabaseDriver>(&self, framework: Framework<D>) -> EmbedBuilder {
        let mut embed = EmbedBuilder::default();
        let colour = match self.interaction.guild_id {
            Some(guild_id) => framework
                .guild_accent_colour(&guild_id)
                .await
                .map(|x| x.unwrap_or(ACCENT_COLOUR)) // Guild has no accent colour
                .unwrap_or(ACCENT_COLOUR), // We had an error getting the guild's accent colour
            None => ACCENT_COLOUR // There is no guild for this interaction
        };
        embed.colour(colour);
        embed
    }
}
