use tracing::warn;

use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::framework::LuroFramework;
use crate::models::LuroResponse;

impl LuroFramework {
    pub async fn server_owner_response(&self, slash: &mut LuroResponse) -> anyhow::Result<()> {
        slash.embed(server_owner_embed().build())?;
        self.respond(slash).await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn server_owner_embed() -> EmbedBuilder {
    warn!("Someone tried to fuck with the server owner using the bot");
    EmbedBuilder::new()
        .color(COLOUR_DANGER)
        .title("That's the server owner you idiot")
        .description("Congratulations moron, that's the server owner. Do you really think I'm gonna try to kick OR ban them? Holy shit, no.")
}
