use tracing::error;
use twilight_util::builder::embed::EmbedBuilder;

use crate::COLOUR_DANGER;

use crate::models::LuroSlash;

impl LuroSlash {
    /// A response returned by default when a command does not exist within Luro.
    pub async fn not_implemented_response(mut self) -> anyhow::Result<()> {
        self.embed(not_implemented_embed().build())?.ephemeral().respond().await
    }
}

/// Returns an embed containing a standardised error message that we were unable to get the channel that an interaction took place in.
fn not_implemented_embed() -> EmbedBuilder {
    error!("A call was made to a command that does not exist!");
    EmbedBuilder::new()
        .title("Command Not Present")
        .color(COLOUR_DANGER)
        .description("Looks like you managed to find a command that is actively being worked on, as it's not executable at present. If this error persists, might be best to let my owner know :)")
}
