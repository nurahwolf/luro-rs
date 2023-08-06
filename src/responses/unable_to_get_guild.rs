use tracing::warn;

use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::framework::LuroFramework;
use crate::models::LuroResponse;

impl LuroFramework {
    pub async fn unable_to_get_guild_response(&self, slash: &mut LuroResponse) -> anyhow::Result<()> {
        slash.embed(unable_to_get_guild_embed().build())?;
        self.respond(slash).await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn unable_to_get_guild_embed() -> EmbedBuilder {
    warn!("Unable to get the guild the interaction was performed in");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("Unable to get guild")
        .description("Can't fetch information for the guild you are in, sorry. Most likely the Discord API is having a certified `fucky wucky` moment.")
}
